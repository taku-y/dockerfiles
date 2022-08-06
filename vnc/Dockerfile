FROM dorowu/ubuntu-desktop-lxde-vnc:focal-arm64

ENV DEBIAN_FRONTEND noninteractive
RUN echo "Set disable_coredump false" >> /etc/sudo.conf
RUN apt-get update -q && \
    apt-get upgrade -yq && \
    apt-get install -yq wget curl git build-essential vim sudo && \
    rm -rf /var/lib/apt/lists/*

# lsb-release locales bash-completion tzdata gosu && \
# RUN rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --home-dir /home/ubuntu --shell /bin/bash --user-group --groups adm,sudo ubuntu && \
    echo ubuntu:ubuntu | chpasswd && \
    echo "ubuntu ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
COPY lxterminal.conf /lxterminal.conf
COPY desktop.conf /desktop.conf
RUN chmod 777 /*.conf

USER ubuntu
RUN mkdir -p /home/ubuntu/.config/lxterminal && \
    cp /lxterminal.conf /home/ubuntu/.config/lxterminal/
RUN mkdir -p /home/ubuntu/.config/lxsession/LXDE && \
    cp /desktop.conf /home/ubuntu/.config/lxsession/LXDE/
USER root

ENV USER ubuntu
