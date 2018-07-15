#!/bin/bash
docker run -it -p 8000:8000 --rm \
    -v $(pwd)/my_proj:/root/my_proj \
    --name my_yew2 my_yew2
