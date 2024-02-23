# Example plugins

The simplest plugins are shell scripts.

## Successful Plugin

Here's an example of the Bash plugin `success.sh` that echoes the parameter supplied to it:

```bash
#!/bin/bash

if [[ "$#" -ne 1 ]];then
  echo "usage: success.sh <MESSAGE>"
  exit 1
fi


MESSAGE="$1"
echo "{\"success\":{\"result\": \"$MESSAGE\"}}"
```

## Failure Plugin

Here's an example of the Bash plugin `failure.sh` that echoes the parameter supplied to it as an error:

```bash
if [[ "$#" -ne 1 ]];then
  echo "usage: failure.sh <MESSAGE>"
  exit 1
fi

MESSAGE="$1"
echo {\"error\":{\"plugin_name\": \"failure.sh\",\"error\": \"$MESSAGE.\",\"exception\": \"Born to fail\",\"fix\": \"Run the success.sh plugin instead.\"}}
```

Have a look at the [scala-deps](https://github.com/ssanj/scala-deps-zatp) plugin for a working Rust implementation. Some other simple Bash examples can be found in the [plugin folder](https://github.com/ssanj/zat/tree/main/tests/plugins).
