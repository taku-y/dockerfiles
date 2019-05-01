#!/bin/bash
docker run -it --rm \
    --volume $(pwd)/../../../taku-y/primitiv:/root/primitiv \
    --volume $(pwd)/../../../taku-y/primitiv-rust:/root/primitiv-rust \
    --volume $(pwd)/learn-pp-rust:/root/learn-pp-rust \
    --name learn-pp-rust learn-pp-rust
