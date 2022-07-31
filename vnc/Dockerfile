FROM ubuntu:20.04

# https://kamino.hatenablog.com/entry/docker_vnc#sec2
RUN apt update
RUN DEBIAN_FRONTEND=noninteractive apt install -y lxde
RUN apt install -y tigervnc-standalone-server tigervnc-common
RUN apt install -y novnc websockify

COPY .Xresources /tmp/.Xresources
COPY docker-entrypoint.sh /tmp
RUN chmod 777 /tmp/.*
RUN chmod 777 /tmp/docker-entrypoint.sh

# User
RUN useradd -m -s /bin/bash ubuntu
USER ubuntu

RUN mkdir /home/ubuntu/.vnc
RUN cp /tmp/.Xresources /home/ubuntu/.Xresources

ENTRYPOINT ["sh", "/tmp/docker-entrypoint.sh"]
