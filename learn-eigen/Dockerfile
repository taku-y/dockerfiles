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

# clang
RUN wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | sudo apt-key add -
RUN apt-add-repository -y "deb http://apt.llvm.org/xenial/ llvm-toolchain-xenial-6.0 main"
RUN apt-get update
RUN apt-get install -y clang-6.0

# Google Test
# https://www.eriksmistad.no/getting-started-with-google-test-on-ubuntu/
RUN sudo apt-get install libgtest-dev
RUN cd /usr/src/gtest && \
    cmake CMakeLists.txt && \
    make && \
    cp *.a /usr/lib

# Eigen3.3
#$ apt install wget
RUN cd /root && \
    wget http://bitbucket.org/eigen/eigen/get/3.3.4.tar.bz2 && \
    mkdir eigen3 && \
    tar -xf 3.3.4.tar.bz2 -C eigen3 --strip-components=1 && \
    rm 3.3.4.tar.bz2

ENV USER=root

WORKDIR /root
