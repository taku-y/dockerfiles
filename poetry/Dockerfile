FROM nvidia/cuda:11.1-devel-ubuntu20.04

# https://github.com/moby/moby/issues/27988
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
RUN apt update -y && apt upgrade -y

RUN apt install -y --no-install-recommends \
        dialog \
        apt-utils
RUN DEBIAN_FRONTEND=noninteractive && \
    apt update && \
    apt install -y -q --no-install-recommends --fix-missing \
        build-essential \
        git \
        curl \
        vim \
        wget \
        ca-certificates \
        libjpeg-dev \
        libpng-dev \
        libopenblas-dev pbzip2 \
        libcurl4-openssl-dev libssl-dev libxml2-dev
RUN DEBIAN_FRONTEND=noninteractive && \
    apt install -y -q --no-install-recommends --fix-missing \
        libbz2-dev \
        libreadline-dev \
        libsqlite3-dev \
        libopencv-dev \
        tk-dev \
        cmake \
        zlib1g-dev \
        llvm libncurses5-dev libncursesw5-dev \
        xz-utils tk-dev libffi-dev liblzma-dev \
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

# --- pyenv ---
RUN mkdir ~/.pyenv
RUN git clone https://github.com/pyenv/pyenv.git ~/.pyenv

# (Option)pyenvのリリースバージョンをv2.0.3に変更
RUN cd ~/.pynev && checkout v2.0.3

# .bashrcの更新
RUN echo 'export PYENV_ROOT="$HOME/.pyenv"' >> ~/.bashrc
RUN echo 'export PATH="$PYENV_ROOT/bin:$PATH"' >> ~/.bashrc
RUN echo 'eval "$(pyenv init --path)"' >> ~/.bashrc
RUN source ~/.bashrc
