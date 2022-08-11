FROM dorowu/ubuntu-desktop-lxde-vnc:focal-arm64

ENV DEBIAN_FRONTEND noninteractive
RUN echo "Set disable_coredump false" >> /etc/sudo.conf
RUN apt-get update -q && \
    apt-get upgrade -yq && \
    apt-get install -yq wget curl git build-essential vim sudo libssl-dev

# lsb-release locales bash-completion tzdata gosu && \
# RUN rm -rf /var/lib/apt/lists/*

# clang
RUN apt install -y -q libclang-dev

# sdl
RUN apt update -y && \
    apt upgrade -y && \
    DEBIAN_FRONTEND=noninteractive && \
    apt install -y -q --no-install-recommends \
        libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-net-dev libsdl2-ttf-dev \
        libsdl-dev libsdl-image1.2-dev

# zip
RUN apt install -y zip

# swig
RUN apt install -y swig

# python
RUN apt install -y python3.8 python3.8-dev python3.8-distutils python3.8-venv python3-pip

# cmake
RUN apt install -y cmake

# headers required for building libtorch
RUN apt install -y libgoogle-glog-dev libgflags-dev

# Cleanup
RUN rm -rf /var/lib/apt/lists/*

# Add user
RUN useradd --create-home --home-dir /home/ubuntu --shell /bin/bash --user-group --groups adm,sudo ubuntu && \
    echo ubuntu:ubuntu | chpasswd && \
    echo "ubuntu ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
COPY lxterminal.conf /lxterminal.conf
COPY desktop.conf /desktop.conf
RUN chmod 777 /*.conf

# User desktop configs
USER ubuntu
RUN mkdir -p /home/ubuntu/.config/lxterminal && \
    cp /lxterminal.conf /home/ubuntu/.config/lxterminal/
RUN mkdir -p /home/ubuntu/.config/lxsession/LXDE && \
    cp /desktop.conf /home/ubuntu/.config/lxsession/LXDE/

# rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# python
RUN pip3 install --upgrade pip
RUN pip3 install pyyaml typing-extensions
RUN pip3 install torch==1.9.0
RUN pip3 install ipython jupyterlab
RUN pip3 install numpy==1.18.0
RUN pip3 install gym[box2d]==0.25.1

# border
RUN cd $HOME && git clone https://github.com/taku-y/border.git

# .bashrc
RUN echo "export LIBTORCH=$HOME/.local/lib/python3.8/site-packages/torch" >> ~/.bashrc
RUN echo "export LD_LIBRARY_PATH=$HOME/.local/lib/python3.8/site-packages/torch/lib" >> ~/.bashrc
RUN echo "export LIBTORCH_CXX11_ABI=0" >> ~/.bashrc
RUN echo 'export PATH=$HOME/.local/bin:$PATH' >> ~/.bashrc
RUN echo 'export PYTHONPATH=$HOME/border/border-py-gym-env/examples:$PYTHONPATH' >> ~/.bashrc

USER root
ENV USER ubuntu
