#!/bin/bash

if [[ "$#" -ne 1 ]];then
  echo "usage: failure-not-j.sh <MESSAGE>"
  exit 1
fi

MESSAGE="$1"
echo "$MESSAGE"
