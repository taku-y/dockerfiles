#!/bin/bash
docker run -it --rm \
    --volume="/Users/taku-y/data:/root/data" \
    --volume=$(pwd)/workdir:/root/workdir \
    --name learn-nlp learn-nlp bash
