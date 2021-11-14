# docker run -it --rm \
#     --gpus 1 \
#     -e USER=ubuntu \
#     -e PASSWD=ubuntu \
#     -v $HOME/git/github/taku-y/border:/home/ubuntu/border \
#     border

docker run -it --rm \
    -e USER=ubuntu \
    -e PASSWD=ubuntu \
    -v $HOME/git/github/taku-y/border:/home/ubuntu/border \
    border
