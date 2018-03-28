# Tested on
* Ubuntu 16.04
* Nvidia-docker

# Build
```
$ cd dockerfiles/mayavi
$ sh build.sh
```

# Run
```
$ xhost +
$ cd dockerfiles/mayavi
$ sh run.sh
```

# Test (in docker container)
```
$ mayavi2
```
