From nvidia/cuda:9.0-cudnn7-runtime-ubuntu16.04
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root

# Basic tools
RUN apt-get update && \
    apt-get install -y vim cmake lsb-core apt-utils git-all build-essential

# glxgears
RUN apt-get -y update && \
    apt-get -y install mesa-utils

# nvidia-docker hooks
LABEL com.nvidia.volumes.needed="nvidia_driver"
ENV PATH /usr/local/nvidia/bin:${PATH}
ENV LD_LIBRARY_PATH /usr/local/nvidia/lib:/usr/local/nvidia/lib64:${LD_LIBRARY_PATH}

# Anaconda
RUN cd /tmp && \
    curl -O https://repo.continuum.io/archive/Anaconda3-5.0.1-Linux-x86_64.sh && \
    bash Anaconda3-5.0.1-Linux-x86_64.sh -b
ENV PATH=/root/anaconda3/bin:${PATH}

# For PyQT
# See https://github.com/unetbootin/unetbootin/issues/66
ENV QT_X11_NO_MITSHM=1

# OpenAI Gym
RUN mkdir -p /tmp/github
RUN cd /tmp/github && git clone https://github.com/openai/gym.git && cd gym && pip install -e .

# PyBullet
RUN pip install pybullet baselines ruamel.yaml

# Tensorflow
RUN pip install tensorflow-gpu
RUN pip install --upgrade numpy