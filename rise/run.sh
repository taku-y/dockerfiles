#!/bin/bash
docker run --rm -v $HOME:/home/jovyan/work -p 8888:8888 --name rise rise