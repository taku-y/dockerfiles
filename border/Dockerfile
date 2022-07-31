FROM ubuntu:20.04

# https://kamino.hatenablog.com/entry/docker_vnc#sec2
RUN apt update
RUN DEBIAN_FRONTEND=noninteractive apt install -y lxde
RUN apt install -y tigervnc-standalone-server tigervnc-common
RUN apt install -y novnc websockify

# https://github.com/moby/moby/issues/27988
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
RUN apt update -y && apt upgrade -y

RUN DEBIAN_FRONTEND=noninteractive && \
    apt install -y -q --no-install-recommends \
        build-essential \
        git \
        curl \
        vim \
        wget \
        ca-certificates \
        libjpeg-dev \
        libpng-dev \
        libopenblas-dev pbzip2 \
        libcurl4-openssl-dev libssl-dev libxml2-dev \
        libbz2-dev \
        libreadline-dev \
        libsqlite3-dev \
        libopencv-dev \
        tk-dev \
        cmake \
        python3.8 python3.8-dev python3.8-distutils python3.8-venv

# locale
RUN apt install -y -q locales && \
    locale-gen en_US && \
    locale-gen en_US.UTF-8 && \
    update-locale

# clang
RUN apt install -y -q libclang-dev

# sdl
RUN apt update -y && \
    apt upgrade -y && \
    DEBIAN_FRONTEND=noninteractive && \
    apt install -y -q --no-install-recommends \
        libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-net-dev libsdl2-ttf-dev \
        libsdl-dev libsdl-image1.2-dev

# zip
RUN apt install -y zip

# swig
RUN apt install -y swig

# User
COPY .Xresources /tmp/.Xresources
COPY docker-entrypoint.sh /tmp
RUN chmod 777 /tmp/.*
RUN chmod 777 /tmp/docker-entrypoint.sh

RUN useradd -m -s /bin/bash ubuntu
USER ubuntu

RUN mkdir /home/ubuntu/.vnc
RUN cp /tmp/.Xresources /home/ubuntu/.Xresources

# pyenv
RUN git clone https://github.com/pyenv/pyenv.git ~/.pyenv
RUN echo 'export PYENV_ROOT="$HOME/.pyenv"' >> ~/.bashrc
RUN echo 'export PATH="$PYENV_ROOT/bin:$PATH"' >> ~/.bashrc
RUN echo 'eval "$(pyenv init --path)"' >> ~/.bashrc
RUN chmod 700 ~/.bashrc
ENV PATH=/home/ubuntu/.pyenv/bin:$PATH
RUN pyenv install 3.10.0
RUN pyenv global 3.10.0
RUN eval "$(pyenv init --path)" && pip install --upgrade pip

# gym
RUN eval "$(pyenv init --path)" && pip install gym[all]
RUN eval "$(pyenv init --path)" && pip install gym[accept-rom-license]

# ipython
RUN eval "$(pyenv init --path)" && pip install ipython

# pybullet-gym
RUN eval "$(pyenv init --path)" && \
    cd ~/ && \
    git clone https://github.com/benelot/pybullet-gym.git && \
    cd pybullet-gym && \
    pip install -e .

# rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# libtorch
RUN mkdir /home/ubuntu/torch
RUN cd /home/ubuntu/torch && \
    wget https://download.pytorch.org/libtorch/cu111/libtorch-cxx11-abi-shared-with-deps-1.9.0%2Bcu111.zip
RUN cd /home/ubuntu/torch && \
    unzip libtorch-cxx11-abi-shared-with-deps-1.9.0+cu111.zip
RUN echo "export LIBTORCH=/home/ubuntu/torch/libtorch" >> ~/.bashrc
RUN echo "export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/ubuntu/torch/libtorch/lib" >> ~/.bashrc

ENTRYPOINT ["sh", "/tmp/docker-entrypoint.sh"]
