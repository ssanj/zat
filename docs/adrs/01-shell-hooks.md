# Shell Hooks

Sometimes it's useful to run a script at after the templates have been rendered to the target directory. It can then do things like making other scripts executable, invoking an API or any other post-rendered functionality that is required.

## Chosen Option

`Option - 2`.

While `Option 3 - In the templates folder and auto executable`, would be less work for the user, it's a little clumsy to implement. `Option 1 - In the templates file folder` doesn't make sense in this instance as the shell script will be retained in the target folder.

## Option 1 - In the templates file folder

- The user creates a `shell-hook.zat-exec` file in the template files directory.
- The user makes the above file executable.
- Copy this file as is (with the execution attribute) when rendering the templates.
- Run this file at the end of the rendering (if it exists)


### Pros

- We can also use template files (`.tmpl`) as the shell script and use variables supplied to it from the `.variables.zat-prompt` file.

### Cons

- More burden on the user has to make the file executable.
- We have to copy this file in a special way to other files, in order to maintain its executable attribute.
- The shell hook is part of the template files directory, and will be present in the target directory. Ideally the target directory should only have files relating to the template - not infrastructure related files.

## Option 2 - In the templates folder

- The user creates a `shell-hook.zat-exec` file in the templates folder.
- The user makes the above file executable.
- Execute this file (if it exists) after the template files have been rendered, on the target folder.

### Pros

- The shell hook file is part of the infrastructure code and will not be copied over to the target.
- We need to run this script if it exists; it has no special handling.

### Cons

- More burden on the user has to make the file executable.


## Option 3 - In the templates folder and auto executable

This is the same as Option 2, except that the `shell-hook.zat-exec` file is made executable by Zat (if it is not).

- The user creates a `shell-hook.zat-exec` file in the templates folder.
- Execute this file (if it exists) after the template files have been rendered, on the target folder.

### Pros

- The shell hook file is part of the infrastructure code and will not be copied over to the target.
- Less burden on the user; they just create the file with it its contents. There's no stuffing around with shell attributes.

### Cons

- We need to have special handling of the shell hook file, in case it's not executable.

## Notes

- The shell hook will run with the target directory as its working directory
- We may need to expose some arguments to the shell script in future

## Questions

- Should the template rendering fail if the shell hook isn't executable or it can't be run or it runs and leads to an error?
> Yes, it should fail because a specific portion of it hasn't run.
