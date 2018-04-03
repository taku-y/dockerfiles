#!/bin/bash
nvidia-docker run -it --rm \
    --env="DISPLAY" \
    --volume="/tmp/.X11-unix:/tmp/.X11-unix:rw" \
    --volume="/home/taku-y:/taku-y" \
    --name my_h5pyviewer my_h5pyviewer bash
