FROM nvidia/cuda:10.2-cudnn7-devel-ubuntu18.04

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

# Haggingface
# https://github.com/huggingface/transformers/blob/master/docker/transformers-pytorch-gpu/Dockerfile
RUN apt update && \
    apt install -y bash \
                   build-essential \
                   git \
                   curl \
                   ca-certificates \
                   python3 \
                   python3-pip && \
    rm -rf /var/lib/apt/lists

RUN python3 -m pip install --no-cache-dir --upgrade pip && \
    python3 -m pip install --no-cache-dir \
    mkl \
    torch

# Install sentencepiece, Laboro-DistilBERT
RUN python3 -m pip install --upgrade pip
# sentencepiece >= 0.1.90
# See https://github.com/google/sentencepiece/issues/500
RUN pip install sentencepiece==0.1.90
# RUN git clone https://github.com/laboroai/Laboro-DistilBERT-Japanese.git

# hidpi
COPY .Xresources /
RUN chmod 777 /.Xresources

# Copy entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh
ENTRYPOINT ["docker-entrypoint.sh"]

# # Prepare Livedoor dataset
# COPY ldcc-20140209.tar.gz /workspace/Laboro-DistilBERT-Japanese/data/livedoor
# RUN cd /workspace/Laboro-DistilBERT-Japanese/data/livedoor && \
#     tar zxf ldcc-20140209.tar.gz

# # Jupyter Lab
# RUN pip install jupyterlab
# RUN jupyter notebook --generate-config
# RUN sed -i "s/[\#]c.NotebookApp.ip = 'localhost'/c.NotebookApp.ip = '0.0.0.0'/g" /root/.jupyter/jupyter_notebook_config.py
# RUN sed -i "s/[\#]c.NotebookApp.allow_root = False/c.NotebookApp.allow_root = True/g" /root/.jupyter/jupyter_notebook_config.py
# RUN sed -i "s/[\#]c.NotebookApp.port = 8888/c.NotebookApp.port = 8891/g" /root/.jupyter/jupyter_notebook_config.py
# RUN sed -i "s/[\#]c.NotebookApp.token = '<generated>'/c.NotebookApp.token = ''/g" /root/.jupyter/jupyter_notebook_config.py

# # Preprocess Livedoor dataset
# RUN cd /workspace/Laboro-DistilBERT-Japanese/data/livedoor && \
#     sed -i "s/python/python3/g" create_corpus_from_index.py

# # Set locale for Japanese
# RUN apt install -y language-pack-ja
# RUN update-locale LANG=ja_JP.UTF-8
# ENV LANG=ja_JP.UTF-8

# # Install tensorflow for tokenization
# RUN pip install tensorflow==2.4.1

# # Laboro distilbert model
# COPY laboro_distilbert.tar /workspace/Laboro-DistilBERT-Japanese/model
# RUN cd /workspace/Laboro-DistilBERT-Japanese/model && tar xf laboro_distilbert.tar

# # Livedoor dataset preprocessing
# RUN cd /workspace/Laboro-DistilBERT-Japanese/data/livedoor && \
#     ./create_corpus_from_index.py text train_indexed.tsv > train.tsv && \
#     ./create_corpus_from_index.py text test_indexed.tsv > test.tsv && \
#     ./create_corpus_from_index.py text dev_indexed.tsv > dev.tsv

# # Upgrade sentensepiece
# RUN pip install sentencepiece==0.1.90

# # Enable Jupyter widget (https://qiita.com/halhorn/items/e8aaf5b63f493f038a53) for using tqdm.notebook
# RUN jupyter nbextension enable --py --sys-prefix widgetsnbextension

