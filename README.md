# Zat

A simple templating system to prevent copy-pasta overload. Zat is inspired by [Giter8](http://www.foundweekends.org/giter8/).

## Installation

### Downloading a Release

Download the latest [release](https://github.com/ssanj/zat/releases) for your operating system (linux or macos).
Make it executable with:

`chmod +x <ZAT_EXEC>`

Copy executable to a directory on your path.


### Building from Source

Ensure you have Cargo and Rust installed.

Run:

```
cargo build --release
Copy binary file from target/release/zat to a directory on your path.
```


## Usage

`zat -h`:

```
A simple templating system to prevent copy-pasta overload

Usage: zat <COMMAND>

Commands:
  process    Process templates defined in a Zat repository
  bootstrap  Generate a minimal bootstrap Zat repository
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

Use `zat --help` for additional usage information.

## Getting Started

To get started generate a bootstrap project and follow the instructions.

General repository structure:

- All Zat configuration files go in the root of the Zat repository. These include the '.variables.zat-prompt' configuration file and the 'shell-hook.zat-exec' shell hook file. The '.variables.zat-prompt' defines any tokens you want replaced. The values for these tokens will be requested from the user when the template is processed. The optional 'shell-hook.zat-exec' file should be an executable file (chmod +x). It will get invoked after the repository has been processed, with single argument of the target directory path. Use this file to handle any post-processing tasks.

- All templated files go in the 'templates' folder under the Zat repository folder. This can include regular files, files and folders with tokenised names and templates.

**Regular files**: Plain old files without any tokens in their name or in their content. These will get copied "as is" to the target directory when the repository is processed. **Note:**
 attributes of a file will not be copied. If you need some attributes maintained for a file, you can do that through a shell hook file.

**Files and folders with tokenised names**: Files and folders with tokens in their name but not in their content. Eg. '$project$_README.md'. These tokens will be replaced when the repository is processed and files and folders will be written to the target directory with the updated names.

**Templates**: Are files that end with a '.tmpl'. Eg. 'README.md.tmpl'. They can have tokens in their name and in their content. The tokens in their names and content will get replaced when the repository is processed. The '.tmpl' suffix is removed when the processed template is written to the target directory.

To create simple bootstrap project run:

```
zat bootstrap --repository-dir <ZAT_REPOSITORY>
```

Once the repository is created, you can run it with:

```
zat process --repository-dir  <ZAT_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

Have a look at the [example tests](https://github.com/ssanj/zat/tree/main/tests/examples) for some sample Zat repositories.

Additional templates can be found in the [Example Templates](#example-templates) section.

## Defining a template

### Folder Structure

Templates should live in a `template` sub folder within the Zat repository.

For example, given a Zat repository of `MyCoolTemplate` with the following structure:

```
MyCoolTemplate /
  .variables.zat-prompt  <-- defines tokens.
  shell-hook.zat-exec    <-- This should be executable and will be run after the repository has been processed successfully.

  template /            <-- folder that contains all template files that should be rendered at the destination
    template-file-1.ext <-- Copied as is
    $project$.ext       <-- Token in this file name will be replaced by values supplied by the user.
    $resource__snake$/  <-- Token in this folder name will be replaced by values supplied by the user.
    README.md.tmpl      <-- Tokens in this template file will be replaced by values supplied by the user.
```


### Defining tokens or variables

Define a file named `.variables.zat-prompt` in the root of your Zat repository. This file will determine the tokens that can be used within the project, either as variable substitutions for file and directory names or file content with a files with the `.tmpl` suffix (templates).

When you `process` the Zat repository, you will be prompted to enter the values of the defined variables.

[Minimal Json schema](dot-variables.schema.json)

Here's the structure of entries in the `variables.prompt` file:


| Field | What is it |
| ----- | ---------- |
| variable_name | Name of the variable to define |
| description |  The description of the variable |
| prompt |What to show the user when prompting for this variable |
| filters | [Optional], The list of [filters](#structure-of-a-filter) available to the variable |
| default_value | [Optional] a default value to use, which the user can override if required |

#### Structure of a filter

A filter is some type of [formatting](#supported-filters) that is applied to the user-supplied value. An example is `Pascal` which converts the value into camel case with all the spaces removed.

`"My variable NAME" -> "MyVariableName"`

See [Supported Filters](#supported-filters) for a full list of supported formats.

| Field | What is it |
| ----- | ---------- |
| name| Name of the filter. `__default__` is used for when this should be the default filter applied. A default filter will not have the filter name appended to the variable name at the usage point. It can simply use the variable name. |
| filter | one of the support filters (see below) |

<details>
  <summary>Sample .variables.zat-prompt file and template</summary>

##### .variables.zat-prompt file:
```json
[
  {
    "variable_name": "project",
    "description": "Name of project",
    "prompt": "Please enter your project name",
    "filters": [
      { "name": "__default__",
        "filter": "Pascal"
      },
      { "name": "underscore",
        "filter": "Snake"
      }
    ]
  },
  {
    "variable_name": "description",
    "description": "What your project is about",
    "prompt": "Please a description of your project",
    "default_value": "Some project description"
  }
]
```

##### Template that uses the above variables:

```
# $project$ <-- default filter for `project`

$description$ <-- variable without filters

Folders will be like: $project__underscore$  <-- variable using the 'underscore' filter on the `project` value, which applies the `Snake` format.
```

Assuming the user supplied the following values for the variables:

|Variable| User-supplied Value|
|---|---|
|project| "My Cool Project"|
|description| [accepts default] |


the template file will render as:

```markdown
# MyCoolProject

Some project description

Folders will be like: my_cool_project
```

</details>

#### Supported filters

Filters modify the user-supplied variables. You can think of a filter as a function from `String` -> `String`.

| Filter | Example |
| ----- | ---------- |
| Camel |  "My variable NAME" -> "myVariableName" |
| Cobol | "My variable NAME" -> "MY-VARIABLE-NAME" |
| Flat | "My variable NAME" -> "myvariablename"   |
| Kebab | "My variable NAME" -> "my-variable-name" |
| Lower | "My variable NAME" -> "my variable name" |
| Noop | "My variable NAME" -> "My variable NAME" |
| Pascal | "My variable NAME" -> "MyVariableName"   |
| Snake | "My variable NAME" -> "my_variable_name" |
| Title | "My variable NAME" -> "My Variable Name" |
| Uppe | "My variable NAME" -> "MY VARIABLE NAME" |


#### Filter usage

Variables can be used in the following:

- File name (eg. ` $project$.sublime-settings`)
- Directory name (eg. `/models/$project$/scaffold.rs`)
- Within a file with a `.tmpl` extension (eg. `$project__python$.py.tmpl`)

A variable with a filter can be used in the following format:

`variable_name` + __ + `filter_name`.

Given a variable named "project" and a filter named "python", we can use it as:

`$project__python$.py.tmpl`

As noted above, if the one of the filter names is `__default__` you can simply use the `variable_name` without appending the filter name.

#### Ignore usage

You can ignore specified files from your `templates` folder using a regular expression. By default any `^.git` files are ignored. You can specify multiple ignored files are directories as:

```
zat process --ignore '^folderX/.*' --ignore '^never.txt' ....
```

Ignored files will not be processed and copied over to the target directory.

### Example templates

| Template | Description |
| ----- | ---------- |
| [Sublime Text Plugin Template](https://github.com/ssanj/st-plugin-zat) | Simple Sublime Text Plugin template |

Also have a look at the [example tests](https://github.com/ssanj/zat/tree/main/tests/examples) for some sample Zat repositories.
