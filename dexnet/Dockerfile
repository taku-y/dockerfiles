From ubuntu:16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# Basic tools
RUN apt-get update && \
    apt-get install -y vim cmake lsb-core apt-utils git-all

# glxgears
RUN apt-get -y update && \
    apt-get -y install mesa-utils libglu1-mesa libvtk5-dev # mayavi2

# nvidia-docker hooks
LABEL com.nvidia.volumes.needed="nvidia_driver"
ENV PATH /usr/local/nvidia/bin:${PATH}
ENV LD_LIBRARY_PATH /usr/local/nvidia/lib:/usr/local/nvidia/lib64:${LD_LIBRARY_PATH}

# Python2
RUN apt-get -y install build-essential python-dev python-pip python-vtk
RUN pip install --upgrade pip
RUN pip install numpy
#RUN apt-get -y install python-numpy python-scipy python-matplotlib ipython ipython-notebook python-pandas python-sympy python-nose
RUN apt-get -y install python-qt4

# For PyQT
# See https://github.com/unetbootin/unetbootin/issues/66
ENV QT_X11_NO_MITSHM=1

# sudo
RUN apt-get update && \
    apt-get -y install sudo
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

# Boost.Numpy etc.
RUN apt-get update && \
    apt-get -y install python-dev libboost-dev libboost-python-dev libboost-system-dev \
               assimp-utils libassimp-dev libspatialindex-dev
RUN apt-get update && \
    apt-get -y install libxml2-dev libxslt1-dev zlib1g-dev
RUN apt-get update && \
    apt-get -y install python-tk

# Dexnet
RUN apt-get -y install libgeos-dev
RUN mkdir /git
RUN cd /git && \
    git clone https://github.com/BerkeleyAutomation/dex-net.git && \
    cd /git/dex-net && \
    git checkout custom-databases
COPY install.sh /git/dex-net
RUN cd /git/dex-net && \
    sh install.sh cpu python

