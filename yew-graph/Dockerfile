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

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

# yew
RUN source /root/.cargo/env && \
    rustup default nightly && \
    cargo install cargo-web
RUN cd /root && \
    git clone https://github.com/DenisKolodin/yew.git

# nvm
RUN apt-get update && \
    apt-get install build-essential libssl-dev && \
    cd /tmp && \
    curl -sL https://raw.githubusercontent.com/creationix/nvm/v0.33.8/install.sh -o install_nvm.sh && \
    bash install_nvm.sh

# node.js
RUN /bin/bash -c 'source ~/.nvm/nvm.sh && nvm install 8.9.4 && nvm alias default 8.9.4'

# Plotly
RUN /bin/bash -c 'source ~/.nvm/nvm.sh && npm install -g plotly.js-dist'

# Cleanup
RUN rm -rf /var/lib/apt/lists/*

RUN echo "source /root/.cargo/env" >> /root/.bashrc
RUN echo "alias startyew='cargo web start --target=wasm32-unknown-unknown --host=0.0.0.0'" >> /root/.bashrc
RUN echo "alias sy='cargo web start --target=wasm32-unknown-unknown --host=0.0.0.0'" >> /root/.bashrc
