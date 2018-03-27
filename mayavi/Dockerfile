From ubuntu:16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# Basic tools
RUN apt-get update && \
    apt-get install -y vim cmake lsb-core apt-utils git-all

RUN apt-get update && \
    apt-get install -y libglu1-mesa libvtk5-dev mayavi2

# glxgears
RUN apt-get -y update && \
    apt-get -y install mesa-utils

# nvidia-docker hooks
LABEL com.nvidia.volumes.needed="nvidia_driver"
ENV PATH /usr/local/nvidia/bin:${PATH}
ENV LD_LIBRARY_PATH /usr/local/nvidia/lib:/usr/local/nvidia/lib64:${LD_LIBRARY_PATH}

# Python2
RUN apt-get -y install build-essential python-dev python-pip
RUN apt-get -y install python-numpy python-scipy python-matplotlib ipython ipython-notebook python-pandas python-sympy python-nose # python-qt4

# For PyQT
# See https://github.com/unetbootin/unetbootin/issues/66
# ENV QT_X11_NO_MITSHM=1
