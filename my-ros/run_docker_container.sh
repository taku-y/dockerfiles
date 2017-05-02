docker run -it --rm -p 5901:5901 -v /Users/taku-y:/home/taku-y/work \
	--name my-ros my-ros \
    bash -c "vncserver :1 -geometry 1280x800 -depth 24 && tail -F /root/.vnc/*.log"