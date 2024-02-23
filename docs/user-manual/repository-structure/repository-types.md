# Repository Types

There are two types of repositories supported by Zat:

- Local repositories
- Remote repositories

## Local repositories

Local repositories can be processed by the `process` command. A local repository is processed and copied over to the specified target directory:

```
zat process --repository-dir <YOUR_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

![Processing a Bootstrap Project](../../images/zat-process-bootstrap.gif)

## Remote repositories

Remote repositories can be processed by the `process-remote` command. Remote repositories will be `Git` cloned locally and then run as per the local repository workflow. To use remote repositories you need to have `Git` installed and on your `PATH`. The repository URL should be `http(s)` and should not require a password for your `Git` user:

```
zat process-remote --repository-url <YOUR_REMOTE_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

![Processing a Remote Project](../../images/zat-process-remote.gif)
