#!/bin/bash

if [[ "$#" -ne 1 ]];then
  echo "usage: failure-not-json.sh <MESSAGE>"
  exit 1
fi

MESSAGE="$1"
echo "$MESSAGE"
