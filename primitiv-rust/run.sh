#!/bin/bash
docker run -it --rm \
    --volume $(pwd)/workspace:/root/workspace \
    --name my_rust my_rust
