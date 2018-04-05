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

# Dependencies
RUN apt-get update && \
    apt-get install -y g++ ipython minizip python-scipy python-sympy qt4-dev-tools
RUN apt-get update && \
    apt-get install -y libassimp-dev libavcodec-dev libavformat-dev libavformat-dev \
                       libboost-all-dev libboost-date-time-dev libbullet-dev libfaac-dev \
                       libglew-dev libgsm1-dev liblapack-dev liblog4cxx-dev libmpfr-dev \
                       libode-dev libogg-dev libpcrecpp0v5 libpcre3-dev libqhull-dev \
                       libqt4-dev libsoqt-dev-common libsoqt4-dev libswscale-dev \
                       libswscale-dev libvorbis-dev libx264-dev libxml2-dev libxvidcore-dev

# collada-dom
RUN mkdir /git && cd /git && \
    git clone https://github.com/rdiankov/collada-dom.git && \
    cd collada-dom && mkdir build && cd build
RUN cd /git/collada-dom/build && cmake .. && make -j4 && make install

# OpenSceneGraph
RUN apt-get install -y libcairo2-dev libjasper-dev libpoppler-glib-dev libsdl2-dev \
                       libtiff5-dev libxrandr-dev && \
    cd /git && \
    git clone --branch OpenSceneGraph-3.4 https://github.com/openscenegraph/OpenSceneGraph.git && \
    cd OpenSceneGraph && mkdir build && cd build && \
    cmake .. -DDESIRED_QT_VERSION=4 && make -j4 && make install

# Flexible Collision Library
RUN apt-get install -y libccd-dev && \
    cd /git && \
    git clone https://github.com/flexible-collision-library/fcl.git && \
    cd fcl && \
    git checkout 0.5.0 && \
    mkdir build && cd build && cmake .. && make -j4 && make install && \
    ln -sf /usr/include/eigen3/Eigen /usr/include/Eigen

# OpenRAVE
# https://scaron.info/teaching/installing-openrave-on-ubuntu-16.04.html
RUN cd /git && \
    git clone --branch latest_stable --progress --verbose https://github.com/rdiankov/openrave.git && \
    cd openrave && \
    git checkout 9c79ea260e1c009b0a6f7c03ec34f59629ccbe2c && \
    mkdir build && cd build && \
    cmake .. -DOSG_DIR=/usr/local/lib64/ && make -j4 && make install

# Visualization
#RUN apt-get install -y libcanberra-gtk-module libcanberra-gtk3-module

# Dexnet
RUN apt-get -y install libgeos-dev
RUN pip install pyglet==1.4.0a1 # visualization requires this pyglet version in 20180405
RUN cd /git && \
    git clone https://github.com/BerkeleyAutomation/dex-net.git && \
    cd dex-net && git checkout custom-databases
COPY install.sh /git/dex-net
RUN cd /git/dex-net && \
    sh install.sh cpu python
RUN pip install visualization

