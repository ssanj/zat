# Remote Checkouts

When cloning remote Git repositories, there is a question around where they should be checked out to.

## Chosen Option
`Option 2 - Use a temporary directory`.

It keeps everything simple: error-recovery, updates and the user not having to clean up folders after multiple checkouts. This is also the technique used by [Giter8](https://github.com/foundweekends/giter8/blob/2ed47ff2437e10dbc611c1936c8d166a68504a6a/cli-git/src/main/scala/Runner.scala#L47). The only downside is offline caching of templates.

## Option  1 - Create a Zat home directory

This involves create a Zat home directory, possibly `~/.zat`, which will store all checked out remote repositories in the form: `domain/repo/user`. For example for the [st-plugin-zat](https://github.com/ssanj/st-plugin-zat) template, it would create the following folder: `~/.zat/github.com/ssanj/st-plugin-zat`. This would give us the opportunity to cache checkouts for offline reuse.

One issue with this is how to recover from a failed checkout since the directories are reused. Handling updates to cached repositories when the remote repository changes could be challenging; do we `git pull` or delete the directory and recreate it? If so how? Does the user supply a new flag indicating when to invalidate a cached repository?

### Pros
- All the checkouts are in one central location
- A previously checked out template can be simply reused/cached without re-checking it out. This would save time and bandwidth should you need to checkout a few templates many times.

### Cons
- Recovery from failed checkouts can get complicated (folder deletion, user intervention etc).
- We'd have to handle updates to previous checkouts that have been updated in the repository. This could get complicated.


## Option  2 - Use a temporary directory

Every remote repository will be checked out to a unique temporary directory. There will be no opportunity for offline caching of templates.

### Pros

- In the event of a checkout error, no recovery needs to be done as there are no shared folders. Every checkout gets a separate folder.
- No need to handle the deletion of directories, as the temporary folders are cleaned up by the OS.
- No need to handle updates to remote repositories separately; Every checkout gets the latest version.

### Cons

- No opportunity for caching
