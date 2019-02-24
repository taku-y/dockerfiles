#!/bin/bash
docker run -it --rm \
    --volume $(pwd)/workspace:/root/workspace \
    --name my_primitiv my_primitiv
