From osrf/ros:indigo-desktop-full 
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root
ENV USER root
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y lxde-core lxterminal \
    tightvncserver language-pack-ja fonts-vlgothic \
    && rm -rf /var/lib/apt/lists/*
RUN useradd --password taku-y -ms /bin/bash taku-y

RUN apt-get install -y python-rosinstall

USER taku-y
RUN echo "source /opt/ros/indigo/setup.bash" >> ~/.bashrc && \
    mkdir ~/work
RUN echo "ROS_PACKAGE_PATH=/home/taku-y/work/ros:\$ROS_PACKAGE_PATH" >> ~/.bashrc
