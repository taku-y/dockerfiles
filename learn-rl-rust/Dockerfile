FROM nvidia/cuda:10.2-cudnn7-devel-ubuntu18.04

RUN apt-get update && apt-get install -y wget libssl-dev curl sudo \
    vim iputils-ping bash build-essential libssl-dev libffi-dev git \
    libxml2-dev libxslt1-dev zlib1g-dev ca-certificates libjpeg-dev cmake

RUN apt-get update && apt-get install -y lxde xrdp
RUN passwd ubuntu
