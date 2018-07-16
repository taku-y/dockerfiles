```bash
$ sh build.sh
$ sh run.sh

# In container
$ cd ~/yew/example/counter
# Below is equivalent to 'cargo web start --target=wasm32-unknown-unknown --host=0.0.0.0'
# 'startyew' is just an alias, see Dockerfile
$ startyew
```

TODO: learn the mechanism of dynamic update of plots in training process of tensorboard
