FROM ubuntu:20.04

# https://kamino.hatenablog.com/entry/docker_vnc#sec2
RUN apt update
RUN DEBIAN_FRONTEND=noninteractive apt install -y lxde
RUN apt install -y tigervnc-standalone-server tigervnc-common
RUN apt install -y novnc websockify

# COPY passwd /root/.vnc/passwd
# COPY .Xresources /root/.Xresources
# RUN chmod 600 /root/.vnc/passwd

# COPY docker-entrypoint.sh /tmp
# RUN chmod 777 /tmp/docker-entrypoint.sh

# COPY passwd /tmp/passwd
COPY .Xresources /tmp/.Xresources
RUN chmod 777 /tmp/.*
COPY docker-entrypoint.sh /tmp
RUN chmod 777 /tmp/docker-entrypoint.sh

# User
# RUN useradd -m -s /bin/bash -u $UID -g $GID $USERNAME
RUN useradd -m -s /bin/bash ubuntu
USER ubuntu

RUN mkdir /home/ubuntu/.vnc
# RUN cp /tmp/passwd /home/ubuntu/.vnc/passwd
RUN cp /tmp/.Xresources /home/ubuntu/.Xresources

ENTRYPOINT ["sh", "/tmp/docker-entrypoint.sh"]
