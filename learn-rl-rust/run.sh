#!/bin/bash
# docker run -it --rm \
#     --gpus 1 \
#     -p 3389:3389 \
#     -u $(id -u):$(id -g) \
#     -e USER=ubuntu \
#     -e PASSWD=ubuntu \
#     learn-rl-rust

docker run -it --rm \
    -p 3389:3389 \
    -p 22:22 \
    -u $(id -u):$(id -g) \
    -e USER=ubuntu \
    -e PASSWD=ubuntu \
    -v /Users/taku-y/git/github/taku-y/learn-rl-rust:/home/ubuntu/lrr \
    learn-rl-rust
