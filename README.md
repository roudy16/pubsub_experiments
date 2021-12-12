This repo is intended to house code and experiments for the purpose of performance
testing various pubsub solutions.

### Env Setup
```
./setup_env.sh
```

### Redis
Install from source: [redis docs](https://redis.io/download#from-source-code)
```
cd ./3rd_party/redis
wget https://download.redis.io/releases/redis-6.2.6.tar.gz
tar xzf redis-6.2.6.tar.gz
cd redis-6.2.6
make
```

### RabbitMQ
Using docker container with out-of-box config
```
docker run -it --rm --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3.8-management
```
