From nvidia/cuda:9.0-cudnn7-runtime-ubuntu16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# Basic tools
RUN apt-get update && \
    apt-get install -y vim cmake lsb-core apt-utils git-all build-essential \
    libncursesw5-dev libgdbm-dev libc6-dev zlib1g-dev libsqlite3-dev tk-dev \
    libssl-dev openssl libbz2-dev libreadline-dev

# Python3
RUN apt-get install -y python3-pip python3-dev python3-venv
RUN apt-get install -y python-pip python-dev

# Python venv
WORKDIR /root
RUN python3 -m venv venv

# Enable source command
RUN mv /bin/sh /bin/sh_tmp && ln -s /bin/bash /bin/sh

# gensim
RUN source /root/venv/bin/activate && \
    pip install gensim

# WikipediaExtractor
RUN source /root/venv/bin/activate && \
    git clone https://github.com/attardi/wikiextractor.git

# Mecab
RUN apt-get install -y mecab libmecab-dev mecab-ipadic mecab-ipadic-utf8
RUN source /root/venv/bin/activate && \
    pip install mecab-python3

# NEologd
# sudo is required to install NEologd
RUN git clone --depth 1 https://github.com/neologd/mecab-ipadic-neologd.git
RUN apt-get update && apt-get -y install sudo
RUN cd /root/mecab-ipadic-neologd && \
    ./bin/install-mecab-ipadic-neologd -n -y

# Juman

# sentencepiece

# Activate venv
RUN echo "source ~/venv/bin/activate" >> ~/.bashrc

# Disable source command
RUN rm /bin/sh && mv /bin/sh_tmp /bin/sh
