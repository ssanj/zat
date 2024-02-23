# Repository Structure

- All Zat configuration files go in the root of the Zat repository. These include the '.variables.zat-prompt' configuration file and the 'shell-hook.zat-exec' shell hook file. The '.variables.zat-prompt' defines any tokens you want replaced. The values for these tokens will be requested from the user when the template is processed. The optional 'shell-hook.zat-exec' file should be an executable file (chmod +x). It will get invoked after the repository has been processed, with single argument of the target directory path. Use this file to handle any post-processing tasks.

- All templated files go in the 'templates' folder under the Zat repository folder. This can include regular files, files and folders with tokenised names and templates.

**Regular files**: Plain old files without any tokens in their name or in their content. These will get copied "as is" to the target directory when the repository is processed. **Note:**
 attributes of a file will not be copied. If you need some attributes maintained for a file, you can do that through a shell hook file.

**Files and folders with tokenised names**: Files and folders with tokens in their name but not in their content. Eg. '$project$_README.md'. These tokens will be replaced when the repository is processed and files and folders will be written to the target directory with the updated names.

**Templates**: Are files that end with a '.tmpl'. Eg. 'README.md.tmpl'. They can have tokens in their name and in their content. The tokens in their names and content will get replaced when the repository is processed. The '.tmpl' suffix is removed when the processed template is written to the target directory.

## Repository Types

There are two types of repositories supported by Zat:

- Local repositories
- Remote repositories

### Local repositories

Local repositories can be processed by the `process` command. A local repository is processed and copied over to the specified target directory:

```
zat process --repository-dir <YOUR_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

### Remote repositories

Remote repositories can be processed by the `process-remote` command. Remote repositories will be `Git` cloned locally and then run as per the local repository workflow. To use remote repositories you need to have `Git` installed and on your `PATH`. The repository URL should be `http(s)` and should not require a password for your `Git` user:

```
zat process-remote --repository-url <YOUR_REMOTE_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

![Processing a Remote Project](docs/images/zat-process-remote.gif)
