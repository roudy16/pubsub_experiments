#!/bin/bash

SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

REDIS_DIR="${SCRIPT_DIR}/3rd_party/redis"
RABBITMQ_DIR="${SCRIPT_DIR}/3rd_party/rabbitmq"
KAFKA_DIR="${SCRIPT_DIR}/3rd_party/kafka"

mkdir -p ${REDIS_DIR}
mkdir -p ${RABBITMQ_DIR}
mkdir -p ${KAFKA_DIR}
