#!/bin/bash
docker run -it --rm \
    --volume $(pwd)/learn-pp-rust:/root/learn-pp-rust \
    --name learn-pp-rust learn-pp-rust
