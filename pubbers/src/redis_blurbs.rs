use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

use anyhow::Error;

use redis::{Commands, ControlFlow, Msg, PubSubCommands, RedisResult};

const CHANNEL: &str = "test_channel";

pub fn redis_blurb_01() -> Result<(), Error> {
    let (tx, rx) = mpsc::channel::<Instant>();

    let sub_loop = spawn(move || {
        sub_job_01(rx);
    });

    let pub_loop = spawn(move || {
        pub_job_01(tx);
    });

    pub_loop.join();
    sub_loop.join();

    return Ok(());
}

fn sub_job_01(rx: Receiver<Instant>) -> () {
    let client = redis::Client::open("redis://localhost/").unwrap();
    let mut con = client.get_connection().unwrap();
    con.set_read_timeout(Some(Duration::from_secs(1)));

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

    return;
}

fn pub_job_01(tx: Sender<Instant>) -> () {
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

    return;
}
