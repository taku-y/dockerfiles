#!/bin/bash
# docker run -it --rm \
#     --gpus 1 \
#     -p 3389:3389 \
#     -u $(id -u):$(id -g) \
#     -e USER=ubuntu \
#     -e PASSWD=ubuntu \
#     learn-rl-rust

docker run -it --rm \
    --gpus 1 \
    -p 3389:3389 \
    -u $(id -u):$(id -g) \
    -e USER=ubuntu \
    -e PASSWD=ubuntu \
    -v /Users/taku-y/git/github/taku-y/border:/home/ubuntu/border \
    learn-rl-rust
