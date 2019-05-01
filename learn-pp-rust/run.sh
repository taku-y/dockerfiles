#!/bin/bash
dir_libtorch=/Users/taku-y/data/downloads/libtorch
dir_tch_rs=$(pwd)/../../../LaurentMazare/tch-rs

docker run -it --rm \
    --volume $(pwd)/../../../taku-y/primitiv:/root/primitiv \
    --volume $(pwd)/../../../taku-y/primitiv-rust:/root/primitiv-rust \
    --volume $(pwd)/learn-pp-rust:/root/learn-pp-rust \
    --volume ${dir_libtorch}:/root/libtorch \
    --volume ${dir_tch_rs}:/root/tch-rs \
    --name learn-pp-rust learn-pp-rust
