#!/bin/bash
docker run -it --rm \
    --volume $(pwd)/workspace:/root/workspace \
    --volume $(pwd)/../../../primitiv/primitiv:/root/primitiv \
    --name my_primitiv my_primitiv
