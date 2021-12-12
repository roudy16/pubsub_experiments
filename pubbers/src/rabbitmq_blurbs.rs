use async_global_executor;
use futures_lite::stream::StreamExt;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

use anyhow::Error;

use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Consumer, Result as LapinResult,
};

pub fn rabbitmq_blurb_01() -> Result<(), Error> {
    let (tx, rx) = mpsc::channel::<Instant>();

    let rabbit_loop = spawn(move || {
        rabbit_job_01(tx, rx);
    });

    rabbit_loop.join();

    return Ok(());
}

fn rabbit_job_01(tx: Sender<Instant>, rx: Receiver<Instant>) -> LapinResult<()> {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    async_global_executor::block_on(async {
        let conn = Connection::connect(
            &addr,
            ConnectionProperties::default().with_default_executor(8),
        )
            .await?;

        let channel_a = conn.create_channel().await?;
        let channel_b = conn.create_channel().await?;

        let queue = channel_a
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let mut consumer: Consumer = channel_b
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        async_global_executor::spawn(async move {
            while let Some(delivery) = consumer.next().await {
                let (_, delivery) = delivery.expect("error in consumer");

                let now = Instant::now();
                let then = rx.recv().unwrap();
                let span = now.duration_since(then);
                let span_micros = span.as_micros();

                println!("{} us", span_micros);

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("ack");
            }
        }).detach();

        let payload = b"Hello world!";

        loop {
            let now = Instant::now();
            tx.send(now);
            let confirm = channel_a
                .basic_publish(
                    "",
                    "hello",
                    BasicPublishOptions::default(),
                    payload.to_vec(),
                    BasicProperties::default(),
                )
                .await?
                .await?;
            assert_eq!(confirm, Confirmation::NotRequested);
            sleep(Duration::from_millis(100))
        }
    })

    /*
    let _: RedisResult<()> = con.subscribe(&[CHANNEL], move |msg: Msg| {
        let now = Instant::now();
        let res = msg.get_payload();

        match res {
            Ok(_) => (),
            _ => return ControlFlow::Break(()),
        }

        let then = rx.recv().unwrap();
        let span = now.duration_since(then);
        let span_micros = span.as_micros();
        let received: String = res.unwrap();

        println!("{}, {} us", received, span_micros);
        return ControlFlow::Continue;
    });
     */
}

fn pub_job_01(tx: Sender<Instant>) -> () {
    /*
    let client = redis::Client::open("redis://localhost/").unwrap();
    let mut con = client.get_connection().unwrap();

    const MESSAGE: &str = "test_msg";
    //let json = serde_json::to_string(&MESSAGE).unwrap();

    let mut counter = 0 as u32;
    while counter < 100 {
        let now = Instant::now();
        tx.send(now);
        let _: () = con.publish(&CHANNEL, &MESSAGE).unwrap();
        counter = counter + 1;
        sleep(Duration::from_millis(100))
    }
     */

    return;
}
