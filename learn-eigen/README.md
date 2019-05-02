```
$ cd primitiv
$ mkdir build
$ cd build
$ cmake .. -DPRIMITIV_BUILD_C_API=ON -DPRIMITIV_BUILD_TESTS=ON \
           -DEIGEN3_INCLUDE_DIR=/root/eigen3 \
           -DPRIMITIV_USE_EIGEN=ON
$ make
$ make install
```
