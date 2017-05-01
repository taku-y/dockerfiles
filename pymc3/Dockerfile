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

RUN apt-get update && \
    apt-get -yqq install \
      texlive \
      texlive-lang-japanese \
      texlive-fonts-recommended \
      texlive-fonts-extra

USER root
RUN pip install hide_code
RUN jupyter nbextension install --py hide_code
RUN jupyter nbextension enable --py hide_code
RUN jupyter serverextension enable --py hide_code

USER jovyan
RUN pip install pdfkit
RUN pip install gensim 
RUN pip install https://storage.googleapis.com/tensorflow/linux/cpu/tensorflow-0.10.0rc0-cp35-cp35m-linux_x86_64.whl
RUN pip install https://github.com/ipython-contrib/jupyter_contrib_nbextensions/tarball/master
# RUN jupyter contrib nbextension install --user
RUN conda install -c damianavila82 rise
RUN pip install sphinx
RUN pip install sphinx_rtd_theme
RUN pip install numpydoc
RUN pip install nbsphinx
RUN pip install recommonmark
RUN pip install joblib
RUN pip install tqdm
RUN pip install theano

ENV PYTHONPATH /home/jovyan/work/git/github/taku-y/pymc3:$PYTHONPATH
ENV MPLBACKEND Agg
