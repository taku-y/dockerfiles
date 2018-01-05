#!/bin/bash
docker run -d \
    -p 8888:8888 -p 6006:6006 \
    -v /Users/taku-y:/notebooks/taku-y \
    --name my_tf_cpu my_tf_cpu > log.txt
