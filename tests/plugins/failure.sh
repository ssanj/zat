#!/bin/bash

MESSAGE="$1"
echo {\"Error\":{\"plugin_name\": \"failure.sh\",\"error\": \"$MESSAGE.\",\"exception\": \"Born to fail\",\"fix\": \"Run the success.sh plugin instead.\"}}
