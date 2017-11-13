# Tested on
* Ubuntu 16.04
* Nvidia-docker

# Build
```
$ cd dockerfiles/gym
$ sh build.sh
```

# Run
```
$ cd dockerfiles/gym
$ sh run.sh
```

# Test (in docker container)
The following command run a cart pole example with graphically displaying episodes.
```
$ cd /tmp/gym/examples/agents
$ python random_agent.py
```
