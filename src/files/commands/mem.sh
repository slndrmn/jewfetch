echo "memory:" $(free -m | grep Mem | awk '{print $3 "MB / " $2 "MB"}')
