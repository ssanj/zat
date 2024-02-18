#!/bin/bash

if [[ "$#" -ne 1 ]];then
  echo "usage: failure.sh <MESSAGE>"
  exit 1
fi

MESSAGE="$1"
echo {\"Error\":{\"plugin_name\": \"failure.sh\",\"error\": \"$MESSAGE.\",\"exception\": \"Born to fail\",\"fix\": \"Run the success.sh plugin instead.\"}}
