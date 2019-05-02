#!/bin/bash
docker run -it --rm \
    --volume $(pwd)/../../../taku-y/learn-eigen:/root/learn-eigen \
    --name my_learn_eigen my_learn_eigen
