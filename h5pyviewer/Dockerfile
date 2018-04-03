From ubuntu:16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# sudo
RUN apt-get update && \
    apt-get -y install sudo
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

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

# https://kokufu.blogspot.jp/2017/04/ubuntu-1604-wxpython-python3.html
RUN apt update && apt-get install -y \
    libgtk-3-dev mesa-common-dev libglu1-mesa-dev python-gst0.10-dev libwebkit-dev libwebkitgtk-dev

# Install h5pyViewer and required packages
RUN pip install h5pyViewer h5py
RUN apt-get install -y libnotify-dev
RUN apt-get install -y libsdl1.2-dev libsdl-image1.2-dev libsdl-mixer1.2-dev libsdl-ttf2.0-dev
RUN pip install matplotlib
RUN apt-get install -y python-tk
RUN pip install cython
RUN pip install glumpy
RUN sudo apt-get install python-opengl python-qt4 python-qt4-gl
RUN apt-get install -y libcanberra-gtk-module libcanberra-gtk3-module
ENV NO_AT_BRIDGE 1
RUN pip install -U \
    -f https://extras.wxpython.org/wxPython4/extras/linux/gtk3/ubuntu-16.04 \
    wxPython
COPY h5pyViewer /usr/local/bin/h5pyViewer
