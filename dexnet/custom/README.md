# Tested on
* Ubuntu 16.04
* Nvidia-docker

# Build
```
$ cd custom
$ sh build.sh
```

# Run
```
$ xhost +
$ cd custom
$ sh run.sh
```

# Test (in docker container; docker exec -it my_dexnet_custom bash)
```
$ cd /git/dex-net
$ python app/dexnet_cli.py
```
