From ubuntu:16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# Basic tools
RUN apt-get update && \
    apt-get install -y vim cmake lsb-core apt-utils git-all wget

# glxgears
RUN apt-get -y update && \
    apt-get -y install mesa-utils libglu1-mesa libvtk5-dev libgl1-mesa-glx

# For PyQT
# See https://github.com/unetbootin/unetbootin/issues/66
ENV QT_X11_NO_MITSHM=1

# xclock
RUN apt-get install -y x11-apps

# sudo
RUN apt-get update && \
    apt-get -y install sudo
RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

# V-HACD
RUN mkdir /git && cd /git && \
    git clone https://github.com/jeffmahler/v-hacd.git && \
    cd v-hacd && \
    sh install.sh

CMD ["/git/v-hacd/src/build/test/testVHACD"]
