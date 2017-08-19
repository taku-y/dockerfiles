From jupyter/datascience-notebook 
MAINTAINER Taku Yoshioka <taku.yoshioka.4096@gmail.com>

USER root
RUN apt-get update && \
    apt-get install -y apt-utils && \
    apt-get install -y graphviz-dev graphviz && \
    apt-get install -y net-tools && \
    apt-get install -y traceroute && \
    apt-get install -y lsof && \
    apt-get install -y ssh

USER root
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

USER jovyan
RUN source activate python2 && \
    pip install gensim

ENV MPLBACKEND Agg
