# Zat

A simple templating system to prevent copy-pasta overload. Zat is inspired by [Giter8](http://www.foundweekends.org/giter8/).

## Installation

### Building from Source

Ensure you have Cargo and Rust installed.

Run:

```
cargo build --release
Copy binary file from target/release/zat to a directory on your path.
```


## Usage

```
zat 0.3.1
Sanj Sahayam
A simple templating system to prevent copy-pasta overload

USAGE:
    zat [OPTIONS] --template-dir <TEMPLATE_DIR> --target-dir <TARGET_DIR>

OPTIONS:
    -h, --help
            Print help information

        --ignores <IGNORES>
            One or more files ignore. Supply multiple times for different files or folders. The
            files '.variables.zat-prompt' and '.git' are always specified. Accepts any valid regular
            expressions

        --target-dir <TARGET_DIR>
            Where to extract the template to

        --template-dir <TEMPLATE_DIR>
            The location of the template

    -V, --version
            Print version information

```


## Defining a template

### Folder Structure

Templates should live in a `template` folder under the supplied *--template-dir* source directory supplied to `zat`.

For example, if your *--template-dir* directory was `MyCoolTemplate`, then `zat` expects the following structure:

```
MyCoolTemplate /
  .varables.zat-prompt  <-- defines any variables that should be used in rendering the template, the values of which will be supplied by the user.
  other-file1.ext  -|
  other-file2.ext   |
  other-folder1/    |> These files and folders will not be rendered at the destination.
  other-folder2/    |
  README.md        -|

  template / <-- folder that contains all template files that should be rendered at the destination
    template-file-1.ext
    template-file-2.ext
    template-file-n.ext
    $project$.ext       <-- variable placeholders in this file name will be replaced by values supplied by the user in conjunction with the contents of .varables.zat-prompt
    $resource__snake$/  <-- variable placeholders in this folder name will be replaced by values supplied by the user in conjunction with the contents of .varables.zat-prompt
    README.md.tmpl <-- variable placeholders in this file will be replaced by values supplied by the user in conjunction with the contents of .varables.zat-prompt
```


### variables prompt

Define a file named `.variables.zat-prompt` in the root of your template folder. This file will determine the variables that can be used within the project, either as variable substitutions for file and directory names or file content with a files with the `.tmpl` suffix.

When you run use template you will be prompted to enter the values of the defined variables.

Here's the structure of entries in the `variables.prompt` file:


| Field | What is it |
| ----- | ---------- |
| variable_name | Name of the variable to define |
| description |  The description of the variable |
| prompt |What to show the user when prompting for this variable |
| filters | [Optional], The list of *filter*s available to the variable |
| default_value | [Optional] a default value to use, which the user can override if required |

#### Structure of a filter

| Field | What is it |
| ----- | ---------- |
| name| Name of the filter. **__default__** is used for when this should be the default filter applied |
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
# $project$

$description$

Folders will be like: $project__underscore$
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

Filters can be used in the following:

- File name (eg. ` $project$.sublime-settings`)
- Directory name (eg. `/models/$project$/scaffold.rs`)
- Within a file with a `.tmpl` extension (eg. `$project__python$.py.tmpl`)

A variable with a filter can be used in the following format:

`variable_name` + __ + `filter_name`.

Given a variable named "project" and a filter named "python", we can use it as:

`$project__python$.py.tmpl`

### Example templates

| Template | Description |
| ----- | ---------- |
| [Sublime Text Plugin Template](https://github.com/ssanj/st-plugin-zat) | Simple Sublime Text Plugin template |
