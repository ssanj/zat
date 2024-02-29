# Defining a Template

Templates should live in a `template` sub folder within the Zat repository.

For example, given a Zat repository of `MyCoolTemplate` with the following structure:

```
MyCoolTemplate /        <-- Zat repository.
  .variables.zat-prompt <-- defines tokens.
  shell-hook.zat-exec   <-- This (optional file) should be executable (if present) and will be run after the repository has been processed successfully.

  template /            <-- folder that contains all template files that should be rendered at the destination.
    template-file-1.ext <-- Copied as is
    $project$.ext       <-- Token in this file name will be replaced by values supplied by the user.
    $resource__snake$/  <-- Token in this folder name will be replaced by values supplied by the user.
    README.md.tmpl      <-- Tokens in this template file will be replaced by values supplied by the user.
    $project$.conf.tmpl <-- Tokens in this template file name and content will be replaced by values supplied by the user.
```

- [Defining tokens or variables](defining-a-template/defining-tokens.md)
- [Defining a filter](defining-a-template/structure-of-a-filter.md)
- [Example configuration](defining-a-template/example-configuration.md)
- [How ignore files and folders](defining-a-template/how-to-ignore-files-and-folders.md)
- [Example templates](defining-a-template/example-templates.md)
- [Defining a shell hook](defining-a-template/shell-hooks.md)
- [Defining a plugin](defining-a-template/plugins.md)
- [Defining a choice](defining-a-template/choices.md)
