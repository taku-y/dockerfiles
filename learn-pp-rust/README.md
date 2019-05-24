# Docker for learn-pp-rust

## Setup primitiv-rust
0. Clone github repositories
```bash
$ mkdir -p ~/git/github/primitiv
$ mkdir -p ~/git/github/taku-y
$ cd ~/git/github/primitiv
$ git clone https://github.com/primitiv/primitiv.git
$ cd ~/git/github/taku-y
$ git clone https://github.com/taku-y/primitiv-rust --branch for-changes-status
$ git clone https://github.com/taku-y/dockerfiles
$ cd dockerfiles
$ git submodule init
$ git submodule update
``` 

1. Run docker container
```bash
$ cd ~/git/github/taku-y/dockerfiles/learn-pp-rust
$ sh build.sh
$ sh run.sh
$ ls
data  eigen3  learn-pp-rust  libtorch  primitiv  primitiv-rust  tch-rs
```

2. Install primitiv
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

3. Run example in learn-pp-rust
```bash
# Inside docker container
$ cd ~/learn-pp-rust
$ ls
Cargo.lock  Cargo.toml  LICENSE  README.md  examples  plot_result.ipynb  src
$ cargo run --example example3
epoch = 0, loss = 24.971493
epoch = 10, loss = 3.25526
epoch = 20, loss = 3.296376
epoch = 30, loss = 2.5736012
epoch = 40, loss = 2.226811
epoch = 50, loss = 2.6029427
epoch = 60, loss = 2.2346075
epoch = 70, loss = 2.681183
epoch = 80, loss = 2.7190642
epoch = 90, loss = 3.7381485
# result.yaml will be created
# In order to plot the result
$ jupyter-notebook --ip=0.0.0.0 --allow-root --port=8890
```
