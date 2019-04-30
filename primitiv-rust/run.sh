#!/bin/bash
docker run -it --rm \
    --volume $(pwd)/../../../taku-y/primitiv:/root/primitiv \
    --volume $(pwd)/../../../taku-y/primitiv-rust:/root/primitiv-rust \
    --name my_rust my_rust
