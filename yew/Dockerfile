From nvidia/cuda:9.0-cudnn7-devel-ubuntu16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# source
# https://stackoverflow.com/questions/20635472
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

# Basic tools
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
            vim cmake lsb-core apt-utils git-all wget x11-apps \
            ca-certificates curl file \
            build-essential \
            autoconf automake autotools-dev libtool xutils-dev \
            software-properties-common python-software-properties

# glxgears
RUN apt-get -y update && \
    apt-get -y install mesa-utils libglu1-mesa libvtk5-dev libgl1-mesa-glx

# See https://github.com/unetbootin/unetbootin/issues/66
ENV QT_X11_NO_MITSHM=1

# sudo
RUN apt-get update && \
    apt-get -y install sudo
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

# Rustup
# See https://github.com/liuchong/docker-rustup/blob/master/dockerfiles/stable/Dockerfile
ENV SSL_VERSION=1.0.2o

RUN curl https://www.openssl.org/source/openssl-$SSL_VERSION.tar.gz -O && \
    tar -xzf openssl-$SSL_VERSION.tar.gz && \
    cd openssl-$SSL_VERSION && ./config -fPIC && make depend && make install && \
    cd .. && rm -rf openssl-$SSL_VERSION*

ENV OPENSSL_LIB_DIR=/usr/local/ssl/lib \
    OPENSSL_INCLUDE_DIR=/usr/local/ssl/include \
    OPENSSL_STATIC=1

# install toolchain
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH

# emscripten
RUN source /root/.cargo/env && \
    rustup target add wasm32-unknown-emscripten 
RUN cd /root && \
    curl https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz | tar -zxv -C ~/
RUN source /root/.cargo/env && \
    cd /root/emsdk-portable && \
    ./emsdk update && \
    ./emsdk install sdk-incoming-64bit && \
    ./emsdk activate sdk-incoming-64bit

# Yew requires rust nightly build
#RUN source /root/.cargo/env && \
#    rustup default nightly

# Cargo-web
#RUN source /root/.cargo/env && \
#    rustup target add wasm32-unknown-unknown && \
#    cargo install cargo-web

# Yew
#RUN source /root/.cargo/env && \
#    cd /root && \
#    git clone https://github.com/DenisKolodin/yew.git && \
#    cd /root/yew && \
#    cargo web build --target=wasm32-unknown-unknown

# Cleanup
RUN rm -rf /var/lib/apt/lists/*

