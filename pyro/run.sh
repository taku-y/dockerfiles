#!/bin/bash
docker run -it --rm \
    -p 8888:8888 \
    --env="DISPLAY" \
    --volume="/tmp/.X11-unix:/tmp/.X11-unix:rw" \
    --volume="$(pwd)/notebook:/home/jovyan/notebook" \
    --name my_pyro my_pyro bash
