# Caveats

- Zat does not officially support Windows.
- Tokens defined in the `.variables.zat-prompt` file can't be escaped within a template file; The whole purpose of these tokens, is to get replaced within a template file.
- You need to have `Git` installed and on your `PATH` to use process remote repositories. Remote repositories should not need a password for your given `Git` user and should be `http(s)` URLs.
- File attributes are not copied over when a repository is processed. Use the `shell-hook.zat-exec` file to apply any attributes required in post processing.
