#!/bin/bash
docker run -d -v $HOME:/home/jovyan/work -p 8888:8888 --name nbpresent nbpresent