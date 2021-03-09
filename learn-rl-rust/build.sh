#!/bin/bash
docker build -t learn-rl-rust \
    --build-arg USER_ID=$(id -u) \
    --build-arg GROUP_ID=$(id -g) \
    --build-arg PASSWD="ubuntu" .
