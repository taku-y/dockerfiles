From nvidia/cuda:9.0-cudnn7-devel-ubuntu16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# Basic tools
RUN apt-get update && \
    apt-get install -y vim cmake lsb-core apt-utils git-all wget x11-apps

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

# Rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.32.0

RUN set -eux; \
    \
# this "case" statement is generated via "update.sh"
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='c9837990bce0faab4f6f52604311a19bb8d2cde989bea6a7b605c8e526db6f02' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='297661e121048db3906f8c964999f765b4f6848632c0c2cfb6a1e93d99440732' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='a68ac2d400409f485cb22756f0b3217b95449884e1ea6fd9b70522b3c0a929b2' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='27e6109c7b537b92a6c2d45ac941d959606ca26ec501d86085d651892a55d849' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    \
    url="https://static.rust-lang.org/rustup/archive/1.11.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain $RUST_VERSION; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# clang
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
RUN apt-add-repository -y "deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main"
RUN apt-get update
RUN apt-get install -y clang-6.0

# primitiv
RUN cd ~/ && \
    git clone --branch develop https://github.com/primitiv/primitiv/ && \
    cd primitiv && \
    mkdir build && \
    cd build && \
    cmake .. -DPRIMITIV_BUILD_C_API=ON && \
    make && \
    make install

RUN echo "export PRIMITIV_INCLUDE_DIR=/root/primitiv" >> ~/.bashrc && \
    echo "export PRIMITIV_LIBRARY_DIR=/root/primitiv/build/primitiv" >> ~/.bashrc && \
    echo "LD_LIBRARY_PATH=/root/primitiv/build/primitiv:$LD_LIBRARY_PATH" >> ~/.bashrc

# primitiv-rust
RUN cd ~/ && \
    git clone --branch for-changes-status https://github.com/taku-y/primitiv-rust/ && \
    cd primitiv-rust && \
    cargo build

ENV USER=root
WORKDIR /root
