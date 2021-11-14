#!/bin/bash
docker build -t border .
# docker build -t border \
#     --build-arg USER_ID=$(id -u) \
#     --build-arg GROUP_ID=$(id -g) \
#     --build-arg GROUP=${GROUP:-${USER}} \
#     --build-arg PASSWD="ubuntu" .
