# Repository Structure

```
ZatRepository  <-- Configuration files go in the root of this directory
  └── template <-- template files go in this directory
```

## Repository directory

Zat configuration files go in the root of the Zat repository. These include the '.variables.zat-prompt' configuration file and the 'shell-hook.zat-exec' shell hook file.
These files will not get copied to the target directory once the template is processed.

The '.variables.zat-prompt' defines any tokens you want replaced within the content of template files and in file and folder names. The values for these tokens will be requested from the user when the template is processed.

The optional 'shell-hook.zat-exec' file should be an executable file (`chmod +x`). It will get invoked after the repository has been processed, with single argument of the `target directory path`. Use this file to handle any post-processing tasks. See (shell hooks)[defining-a-template/shell-hooks.md] for more information.

## Template directory

All templated files go in the 'templates' folder under the Zat repository folder. This can include regular files, files and folders with tokenised names and templates with tokenised content.

See [Defining a template](defining-a-template.md) for more information.

## Regular files

These are plain old files without any tokens in their name or in their content. These will get copied "as is" to the target directory when the repository is processed.
**Note:** Attributes of a file will not be copied. If you need some attributes maintained for a file, you can do that through a shell hook file.

## Tokenised names

These are files and folders with tokens in their name but not in their content. Eg. '$project$_README.md'. These tokens will be replaced when the repository is processed and the files and folders will be written to the target directory with the updated names.

## Templates

These are files that end with a '.tmpl' suffix. Eg. 'README.md.tmpl'. They can have tokens in their name and in their content. The tokens in their names and content will get replaced when the repository is processed. The '.tmpl' suffix is removed when the processed template is written to the target directory.

- [Repository types](repository-structure/repository-types.md)
