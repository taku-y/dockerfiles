From nvidia/cuda:9.0-cudnn7-devel-ubuntu16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# Basic tools
RUN apt-get update && \
    apt-get install -y vim cmake lsb-core apt-utils git-all wget x11-apps \
    ca-certificates curl file \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev && \
    rm -rf /var/lib/apt/lists/*

# glxgears
RUN apt-get -y update && \
    apt-get -y install mesa-utils libglu1-mesa libvtk5-dev libgl1-mesa-glx

# For PyQT
# See https://github.com/unetbootin/unetbootin/issues/66
ENV QT_X11_NO_MITSHM=1

# sudo
RUN apt-get update && \
    apt-get -y install sudo
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

# add-apt-repository
RUN apt-get install -y software-properties-common python-software-properties

# clang
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
RUN apt-add-repository -y "deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main"
RUN apt-get update
RUN apt-get install -y clang-6.0

# Rust toolchain
RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y

# Google Test
# https://www.eriksmistad.no/getting-started-with-google-test-on-ubuntu/
RUN sudo apt-get install libgtest-dev
RUN cd /usr/src/gtest && \
    cmake CMakeLists.txt && \
    make && \
    cp *.a /usr/lib

# Eigen3.3
RUN cd /root && \
    wget http://bitbucket.org/eigen/eigen/get/3.3.4.tar.bz2 && \
    mkdir eigen3 && \
    tar -xf 3.3.4.tar.bz2 -C eigen3 --strip-components=1 && \
    rm 3.3.4.tar.bz2

# PRIMITIV should be installed under /usr/local/lib
ENV PRIMITIV_LIBRARY_DIR=/usr/local/lib
ENV LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# tch-rs, libtorch binding for Rust
ENV LD_LIBRARY_PATH=/root/libtorch/lib:$LD_LIBRARY_PATH

# Python
RUN apt-get install -y libsqlite3-dev
RUN mkdir -p /tmp/Python37
WORKDIR /tmp/Python37
RUN wget https://www.python.org/ftp/python/3.7.0/Python-3.7.0.tar.xz
RUN tar xvf Python-3.7.0.tar.xz
RUN cd /tmp/Python37/Python-3.7.0 && \
    ./configure --enable-optimizations && \
    make altinstall
RUN wget https://bootstrap.pypa.io/get-pip.py
RUN python3.7 get-pip.py

# RUN add-apt-repository ppa:jonathonf/python-3.6
# RUN apt-get update
# RUN apt-get install -y python3.6 python3.6-dev
RUN pip3 install jupyter numpy matplotlib seaborn ruamel.yaml

# tmux
RUN apt-get install -y tmux

# EvCxR
RUN apt install -y libzmq3-dev
RUN /root/.cargo/bin/cargo install evcxr_jupyter
RUN /root/.cargo/bin/evcxr_jupyter --install

# Config for jupyter notebook
RUN jupyter notebook --generate-config
RUN echo "c.NotebookApp.allow_root = True" >> /root/.jupyter/jupyter_notebook_config.py
RUN echo "c.NotebookApp.ip = '0.0.0.0'" >> /root/.jupyter/jupyter_notebook_config.py
RUN echo "c.NotebookApp.token = ''" >> /root/.jupyter/jupyter_notebook_config.py

ENV PATH=/root/.cargo/bin:$PATH
ENV USER=root
WORKDIR /root
