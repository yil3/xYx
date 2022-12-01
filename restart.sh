#! /bin/bash

if [ "$1" ]; then
  num=$1
else
  echo "process list"
  echo "----------------"
  echo "1.authorization"
  echo "2.user-resource"
  echo "3.authorization-view"
  echo "4.user-resource-view"
  echo "----------------"

  read -p "please input number: " num
fi


case $num in 
  1) if [ -n "$(lsof -i:5000 -t)" ]
      then kill -9 $(lsof -i:5000 -t)
    fi
    # nohup cargo run --bin authorization > /dev/null 2>&1 &
    cargo run --bin authorization
    echo 'restartd authorization'
    ;;
  2) if [ -n "$(lsof -i:5010 -t)" ]
      then kill -9 $(lsof -i:5010 -t)
    fi
    # nohup cargo run --bin user-resources > /dev/null 2>&1 &
    cargo run --bin user-resources
    echo 'restartd user-resources'
    ;;
  3) if [ -n "$(ps -ef |grep node |grep authorize)" ]
      then ps -ef |grep node |grep authorize|awk '{print $2}'|xargs kill -9
    fi
    cd ./view/authorize-view
    nohup yarn dev > /dev/null 2>&1 &
    echo 'restartd authorize-view'
    ;;
  4) if [ -n "$(ps -ef |grep node |grep user-resources)" ]
      then ps -ef |grep node |grep user-resources|awk '{print $2}'|xargs kill -9
    fi
    cd ./view/user-resources-view
    nohup yarn dev > /dev/null 2>&1 &
    echo 'restartd user-resources-view'
    ;;
  *) echo 'please input number 1-4'
    exit 1
    ;;
esac
