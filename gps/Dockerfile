From ubuntu:14.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# glxgears
RUN apt-get -y update && \
    apt-get -y install mesa-utils

# xclock
RUN apt-get -y install xorg openbox

# nvidia-docker hooks
LABEL com.nvidia.volumes.needed="nvidia_driver"
ENV PATH /usr/local/nvidia/bin:${PATH}
ENV LD_LIBRARY_PATH /usr/local/nvidia/lib:/usr/local/nvidia/lib64:${LD_LIBRARY_PATH}

# Python2
RUN apt-get -y install build-essential python-dev python-pip
RUN apt-get -y install python-numpy python-scipy python-matplotlib ipython ipython-notebook python-pandas python-sympy python-nose # python-qt4

# GPS
RUN apt-get install -y libprotobuf-dev protobuf-compiler libboost-all-dev
RUN pip install protobuf
RUN apt-get install -y git
RUN cd / && git clone https://github.com/cbfinn/gps.git && \
    cd gps && ./compile_proto.sh

# Box2D
RUN apt-get install -y swig python-pygame python-qt4
RUN cd / && git clone https://github.com/pybox2d/pybox2d && \
    cd pybox2d python setup.py build && python setup.py install

# For PyQT
# See https://github.com/unetbootin/unetbootin/issues/66
ENV QT_X11_NO_MITSHM=1

# MuJoCo
# RUN apt-get install -y openscenegraph
# RUN apt-get install -y libpython2.7-dev
# RUN cd /gps/build && cmake ../src/3rdparty && make -j4
# ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/path/to/gps/build/lib
# ENV PYTHONPATH=$PYTHONPATH:/path/to/gps/build/lib

# vi
RUN apt-get install -y vim

# CMake
RUN apt-get install -y cmake

# lsb-core
RUN apt-get install -y lsb-core

# apt-utils
RUN apt-get install -y apt-utils

# ROS
RUN sh -c 'echo "deb http://packages.ros.org/ros/ubuntu $(lsb_release -sc) main" > /etc/apt/sources.list.d/ros-latest.list'
RUN apt-key adv --keyserver hkp://ha.pool.sks-keyservers.net:80 --recv-key 421C365BD9FF1F717815A3895523BAEEB01FA116
RUN apt-get update
RUN apt-get install -y ros-indigo-desktop-full
RUN apt-get install -y ros-indigo-gazebo-ros-pkgs ros-indigo-gazebo-ros-control
RUN apt-get install -y ros-indigo-pr2-common
RUN apt-get install -y ros-indigo-pr2-gazebo
RUN rosdep init && rosdep update
RUN echo "source /opt/ros/indigo/setup.bash" >> ~/.bashrc

# GPS-PR2
RUN /bin/bash -c "source /opt/ros/indigo/setup.bash && \
    ROS_PACKAGE_PATH=/opt/ros/indigo/share:/opt/ros/indigo/stacks:/gps:/gps/src/gps_agent_pkg && \
    cd /gps/src/gps_agent_pkg && cmake ."
