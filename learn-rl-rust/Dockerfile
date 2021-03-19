FROM nvidia/cuda:10.2-cudnn7-devel-ubuntu18.04

RUN apt-get update && apt-get install -y wget libssl-dev curl sudo \
    vim iputils-ping bash build-essential libssl-dev libffi-dev git \
    libxml2-dev libxslt1-dev zlib1g-dev ca-certificates libjpeg-dev cmake

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y lxde xrdp

# RUN useradd -d ${HOME} -m -s /bin/bash -u $USER_ID -g $GROUP_ID $USER
RUN useradd -d /home/ubuntu -m -s /bin/bash -u 1000 -g 1000 ubuntu
RUN passwd ubuntu
