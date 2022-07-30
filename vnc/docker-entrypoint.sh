USER=root vncserver :1 -geometry 2560x1200 -depth 24
websockify -D --web=/usr/share/novnc/ 80 localhost:5901
bash
