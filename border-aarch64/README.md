Tested on M2 Macbook Air

sh build.sh
sh run.sh
password: ubuntu
sh remove.sh

python -m robosuite.demos.demo_random_action

cd border
cargo run --example dqn_cartpole --features=tch
