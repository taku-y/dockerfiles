#!/bin/bash
nvidia-docker run -it --rm \
    --env="DISPLAY" \
    --volume="/tmp/.X11-unix:/tmp/.X11-unix:rw" \
    --name my_gym my_gym bash
