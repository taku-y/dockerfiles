# Docker for learn-pp-rust

## Setup primitiv-rust
0. Run docker container
```bash
$ sh build.sh
$ sh run.sh
```

1. Install primitiv
```bash
# Inside docker container
$ cd ~/primitiv
$ mkdir build
$ cd build
$ cmake .. -DPRIMITIV_BUILD_C_API=ON -DPRIMITIV_BUILD_TESTS=ON \
           -DEIGEN3_INCLUDE_DIR=/root/eigen3 \
           -DPRIMITIV_USE_EIGEN=ON
$ make
$ make install
```

`libprimitiv[_c].so` are copied in `/usr/local/lib`.

2. Run example in primitiv-rust
1. Install primitiv
```bash
# Inside docker container
$ cd ~/primitiv-rust
$ cargo run --example xor
```

## Run Examples in learn-pp-rust
