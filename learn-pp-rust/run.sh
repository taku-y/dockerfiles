#!/bin/bash
dir_libtorch=$HOME/data/downloads/libtorch
dir_tch_rs=$HOME/git/github/LaurentMazare/tch-rs
dir_tch_rs_data=$HOME/data/tch-rs-examples
dir_primitiv=$HOME/git/github/primitiv/primitiv
dir_primitiv_rust=$HOME/git/github/taku-y/primitiv-rust

docker run -it --rm \
    --volume ${dir_primitiv}:/root/primitiv \
    --volume ${dir_primitiv_rust}:/root/primitiv-rust \
    --volume $(pwd)/learn-pp-rust:/root/learn-pp-rust \
    --volume ${dir_libtorch}:/root/libtorch \
    --volume ${dir_tch_rs}:/root/tch-rs \
    --volume ${dir_tch_rs_data}:/root/data \
    -p 8890:8890 \
    --name learn-pp-rust learn-pp-rust
