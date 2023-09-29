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
zat 0.1.0
Sanj Sahayam
A simple templating system to prevent copy-pasta overload

USAGE:
    zat --template <TEMPLATE> --destination <DESTINATION>

OPTIONS:
    -d, --destination <DESTINATION>    Where to extract the template to
    -h, --help                         Print help information
    -t, --template <TEMPLATE>          The location of the template
    -V, --version                      Print version information
```


## Defining a template

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

#### Structure of a filter

| Field | What is it |
| ----- | ---------- |
| name| Name of the filter. **__default__** is used for when this should be the default filter applied |
| filter | one of the support filters (see below) |


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
