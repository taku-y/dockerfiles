FROM nvidia/cuda:11.1-devel-ubuntu20.04

# https://github.com/moby/moby/issues/27988
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
RUN apt update -y && apt upgrade -y

RUN apt install -y --no-install-recommends \
        dialog \
        apt-utils
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

# --- locale ---
RUN apt install -y -q locales && \
    locale-gen en_US && \
    locale-gen en_US.UTF-8 && \
    update-locale


# --- clang ---
RUN apt install -y -q libclang-dev

# --- sdl ---
RUN apt update -y && \
    apt upgrade -y && \
    DEBIAN_FRONTEND=noninteractive && \
    apt install -y -q --no-install-recommends \
        libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-net-dev libsdl2-ttf-dev \
        libsdl-dev libsdl-image1.2-dev

# --- zip ---
RUN apt install -y zip

# --- pip ---
RUN apt install -y python3-pip

# --- User ---
# RUN useradd -m -s /bin/bash -u $UID -g $GID $USERNAME
RUN useradd -m -s /bin/bash ubuntu
USER ubuntu

# --- libtorch ---
RUN mkdir /home/ubuntu/torch
RUN cd /home/ubuntu/torch && \
    wget https://download.pytorch.org/libtorch/cu111/libtorch-cxx11-abi-shared-with-deps-1.9.0%2Bcu111.zip
RUN cd /home/ubuntu/torch && \
    unzip libtorch-cxx11-abi-shared-with-deps-1.9.0+cu111.zip

# --- gym-rs for checking build ---
RUN cd /home/ubuntu && \
    git clone https://github.com/elbaro/gym-rs.git

# --- rustup ---
RUN cd /home/ubuntu && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y

# --- gym ---
RUN pip3 install --upgrade pip
RUN pip3 install gym[atari]

# --- env ---
RUN echo "export LIBTORCH=/home/ubuntu/torch/libtorch" >> ~/.bashrc
RUN echo "export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/ubuntu/torch/libtorch/lib" >> ~/.bashrc
