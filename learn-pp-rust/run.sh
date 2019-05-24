#!/bin/bash
dir_libtorch=$HOME/data/downloads/libtorch
dir_tch_rs=$(pwd)/../../../LaurentMazare/tch-rs
dir_tch_rs_data=$HOME/data/tch-rs-examples

docker run -it --rm \
    --volume $(pwd)/../../../taku-y/primitiv:/root/primitiv \
    --volume $(pwd)/../../../taku-y/primitiv-rust:/root/primitiv-rust \
    --volume $(pwd)/learn-pp-rust:/root/learn-pp-rust \
    --volume ${dir_libtorch}:/root/libtorch \
    --volume ${dir_tch_rs}:/root/tch-rs \
    --volume ${dir_tch_rs_data}:/root/data \
    -p 8890:8890 \
    --name learn-pp-rust learn-pp-rust
