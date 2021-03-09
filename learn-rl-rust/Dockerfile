FROM nvidia/cuda:10.2-cudnn7-devel-ubuntu18.04

ARG USER_ID
ARG GROUP_ID
ARG PASSWD
ENV USER ubuntu
# ENV GROUP ${GROUP:-${USER}}

RUN echo 'path-include=/usr/share/locale/ja/LC_MESSAGES/*.mo' > /etc/dpkg/dpkg.cfg.d/includes \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y \
      sudo \
      lxde \
      xrdp \
      ibus \
      ibus-mozc \
      language-pack-ja-base \
      language-pack-ja \
      fonts-noto-cjk \
      fonts-noto-color-emoji \
      supervisor \
      gosu \
    && apt-get clean \
    && rm -rf /var/cache/apt/archives/* \
    && rm -rf /var/lib/apt/lists/*

# # Set locale
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
                   ca-certificates \
                   python3 \
                   python3-pip && \
    rm -rf /var/lib/apt/lists

RUN python3 -m pip install --no-cache-dir --upgrade pip
# RUN python3 -m pip install --no-cache-dir --upgrade pip && \
    # python3 -m pip install --no-cache-dir # torch mkl

# RUN git clone https://github.com/NVIDIA/apex
# RUN cd apex && \
#     python3 setup.py install && \
#     pip install -v --no-cache-dir --global-option="--cpp_ext" --global-option="--cuda_ext" ./

# hidpi
COPY .Xresources /
RUN chmod 777 /.Xresources

# # tqdm
# RUN pip install tqdm

RUN echo $(id -u)

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
RUN cd ${HOME} && curl https://sh.rustup.rs -sSf | sh -s -- -y

USER root

CMD ["/usr/local/bin/_gosu", "root", "/usr/bin/supervisord", "-c", "/etc/supervisor/xrdp.conf"]
