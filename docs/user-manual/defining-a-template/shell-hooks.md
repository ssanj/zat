# Shell hooks

Shell hooks are an executable that gets run after a template has been processed. You can think of it like a 'post-processing' action. The shell hook
file should be named 'shell-hook.zat-exec', should be executable (`chmod +x`) and should be in the root of your Zat repository.

The shell hook file is also optional; it will only get run if it's present.

The shell hook file gets passed in a single parameter value which is the path to the target directory.

Here's an example of a shell hook file that makes all the `.sh` files in the root of the target directory executable:

```bash
#!/bin/bash

TARGET_DIR="$1"

echo "making scripts executable..."
chmod +x "$TARGET_DIR/*.sh"
```

Have a look at the [The Rust Cli Template](https://github.com/ssanj/rust-cli-zat/blob/main/shell-hook.zat-exec) for another example of a shell hook.
