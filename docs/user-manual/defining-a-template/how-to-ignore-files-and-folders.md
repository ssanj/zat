# How to Ignore files and folders

You can ignore specified files from your `templates` folder using a regular expression. By default any `^.git` files are ignored. You can specify multiple ignored files and directories as:

```
zat process --ignore '^folderX/.*' --ignore '^never.txt' ....
```

Ignored files will not be processed or copied over to the target directory.
