#! /bin/bash

echo "process list"
echo "----------------"
echo "1.authorization"
echo "2.user-resource"
echo "3.authorization-view"
echo "4.user-resource-view"
echo "----------------"

echo -n "please input number: "
read num

case $num in 
  1) echo 'restart authorization'
    if [ -n "$(lsof -i:5000 -t)" ]
      then kill -9 $(lsof -i:5000 -t)
    fi
    cd ./authorization
    nohup cargo run > /dev/null 2>&1 &
    ;;
  2) echo 'restart user-resource'
    if [ -n "$(lsof -i:5010 -t)" ]
      then kill -9 $(lsof -i:5010 -t)
    fi
    cd ./user-resources
    nohup cargo run > /dev/null 2>&1 &
    ;;
  3) echo 'restart authorize-view'
    if [ -n "$(lsof -i:3000 -t)" ]
      then kill -9 $(lsof -i:3000 -t)
    fi
    cd ./view/authorize-view
    nohup yarn dev > /dev/null 2>&1 &
    ;;
  4) echo 'restart user-resource-view'
    if [ -n "$(lsof -i:3010 -t)" ]
      then kill -9 $(lsof -i:3010 -t)
    fi
    cd ./view/user-resource-view
    nohup yarn dev > /dev/null 2>&1 &
    ;;
  *) echo 'please input number 1-4'
    exit 1
    ;;
esac
