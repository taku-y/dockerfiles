#!/bin/bash
nvidia-docker run -it --rm \
    -p 8888:8888 -p 6006:6006 \
    -v /home/taku-y:/notebooks/taku-y \
    --name my_tf my_tf
