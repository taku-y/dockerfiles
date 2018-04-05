# Tested on
* Ubuntu 16.04
* Nvidia-docker

# Build
```
$ cd master # this directory
$ sh build.sh
```

# Run
```
$ xhost +
$ cd master
$ sh run.sh
```

# Test (in docker container; docker exec -it my_dexnet_master bash)
```
$ cd /git/dex-net
$ python app/dexnet_cli.py
```
