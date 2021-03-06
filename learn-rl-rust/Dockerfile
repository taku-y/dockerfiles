FROM nvidia/cuda:10.2-cudnn7-devel-ubuntu18.04

ARG USER_ID
ARG GROUP_ID
ARG GROUP
ARG PASSWD
ENV USER ubuntu
# ENV GROUP ${GROUP:-${USER}}

RUN apt-get update && apt-get install -y wget

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y \
      sudo \
      lxde \
      xrdp \
      ibus \
      fonts-noto-cjk \
      fonts-noto-color-emoji \
      supervisor \
      gosu \
      libssl-dev \
    && apt-get clean \
    && rm -rf /var/cache/apt/archives/* \
    && rm -rf /var/lib/apt/lists/*

# # Set locale
# echo 'path-include=/usr/share/locale/ja/LC_MESSAGES/*.mo' > /etc/dpkg/dpkg.cfg.d/includes \
#   language-pack-ja-base \
#   language-pack-ja \
# RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime \
#     && echo 'Asia/Tokyo' > /etc/timezone
# RUN locale-gen ja_JP.UTF-8 \
#     && echo 'LC_ALL=ja_JP.UTF-8' > /etc/default/locale \
#     && echo 'LANG=ja_JP.UTF-8' >> /etc/default/locale
# ENV LANG=ja_JP.UTF-8 \
#     LANGUAGE=ja_JP:ja \
#     LC_ALL=ja_JP.UTF-8

# Set default vars
ENV DEFAULT_USER=developer \
    DEFAULT_PASSWD=xrdppasswd

# Set sudoers for any user
RUN echo "ALL ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers.d/ALL

# Change permission so that non-root user can add users and groups
RUN chmod u+s /usr/sbin/useradd \
    && chmod u+s /usr/sbin/groupadd

# Expose RDP port
EXPOSE 3389

RUN echo "startlxde" > /etc/skel/.xsession \
    && install -o root -g xrdp -m 2775 -d /var/run/xrdp \
    && install -o root -g xrdp -m 3777 -d /var/run/xrdp/sockdir \
    && install -o root -g root -m 0755 -d /var/run/dbus

# Set supervisord conf for xrdp service
RUN { \
      echo "[supervisord]"; \
      echo "user=root"; \
      echo "nodaemon=true"; \
      echo "logfile=/var/log/supervisor/supervisord.log"; \
      echo "childlogdir=/var/log/supervisor"; \
      echo "[program:dbus]"; \
      echo "command=/usr/bin/dbus-daemon --system --nofork --nopidfile"; \
      echo "[program:xrdp-sesman]"; \
      echo "command=/usr/sbin/xrdp-sesman --nodaemon"; \
      echo "[program:xrdp]"; \
      echo "command=/usr/sbin/xrdp --nodaemon"; \
      echo "user=xrdp"; \
    } > /etc/supervisor/xrdp.conf

# vim, ping
RUN apt update
RUN apt install -y vim iputils-ping

# pip
RUN apt update && \
    apt install -y bash \
                   build-essential \
                   git \
                   curl \
                   ca-certificates && \
    rm -rf /var/lib/apt/lists

RUN apt update && \
    apt install -y vim iputils-ping bash build-essential libssl-dev libffi-dev git curl \
    libxml2-dev libxslt1-dev zlib1g-dev ca-certificates libjpeg-dev cmake
RUN apt-get update && \
    apt-get install -y software-properties-common
RUN apt-add-repository ppa:tista/adapta -y && apt update && apt-get install -y adapta-gtk-theme

RUN add-apt-repository ppa:deadsnakes/ppa && \
    apt-get update && \
    apt-get install -y python3.9 python3.9-venv python3.9-dev
RUN curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py && \
    python3.9 get-pip.py
RUN update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.6 1
RUN update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.9 0
RUN update-alternatives --set python3 /usr/bin/python3.9

RUN python3 -m pip install pip
RUN pip3 install gym[atari]
RUN git clone https://github.com/benelot/pybullet-gym.git && cd pybullet-gym && pip3 install -e .

# hidpi
COPY .Xresources /
RUN chmod 777 /.Xresources

# # tqdm
# RUN pip install tqdm

RUN echo $(id -u)

# Add group
RUN groupadd -g $GROUP_ID $GROUP

# Add user
ENV HOME /home/$USER
RUN echo ${USER}
RUN echo ${GROUP_ID}
RUN echo ${USER_ID}
RUN useradd -d ${HOME} -m -s /bin/bash -u $USER_ID -g $GROUP_ID $USER

RUN cp /etc/skel/.xsession ${HOME}/.xsession
RUN sudo install -g $GROUP_ID -m 4750 $(which gosu) /usr/local/bin/_gosu
RUN sudo cp /.Xresources $HOME
RUN echo ${USER}:${PASSWD} | sudo chpasswd

USER ubuntu

# Rustup
RUN cd ${HOME} && curl https://sh.rustup.rs -sSf | sh -s -- -y

# libtorch
# RUN cd ${HOME} && wget https://download.pytorch.org/libtorch/cu110/libtorch-cxx11-abi-shared-with-deps-1.7.1%2Bcu110.zip
# RUN cd ${HOME} && unzip libtorch-cxx11-abi-shared-with-deps-1.7.1+cu110.zip
# RUN cd ${HOME} && wget https://download.pytorch.org/libtorch/cu102/libtorch-cxx11-abi-shared-with-deps-1.7.1.zip
# RUN cd ${HOME} && unzip libtorch-cxx11-abi-shared-with-deps-1.7.1.zip
RUN cd ${HOME} && wget https://download.pytorch.org/libtorch/cu111/libtorch-cxx11-abi-shared-with-deps-1.8.1%2Bcu111.zip
RUN cd ${HOME} && unzip libtorch-cxx11-abi-shared-with-deps-1.8.1+cu111.zip

RUN echo export LIBTORCH=/home/ubuntu/libtorch >> ~/.bashrc
RUN echo export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/ubuntu/libtorch/lib

# .bashrc
RUN echo "setxkbmap -layout us" >> ${HOME}/.bashrc
RUN echo "export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/home/ubuntu/libtorch/lib" >> ${HOME}/.bashrc

USER root

RUN apt-get update && apt-get install -y pkg-config arc-theme

# Swig, Box2d
RUN apt-get install -y swig
RUN pip3 install box2d box2d-py
RUN pip3 install gym[box2d]

# Tensorboard
RUN pip3 install -y tensorboard

CMD ["/usr/local/bin/_gosu", "root", "/usr/bin/supervisord", "-c", "/etc/supervisor/xrdp.conf"]
