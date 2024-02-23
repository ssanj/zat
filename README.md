# Zat

![GitHub Release](https://img.shields.io/github/v/release/ssanj/zat)

A simple project templating system to prevent copy-pasta overload. Zat is inspired by [Giter8](http://www.foundweekends.org/giter8/).

Zat has the following goals:

- It's easy to use
- Has great error messages

## What is a "project templating system"?

If you've ever been working on a project and then required some or all of the same project files and structures in another similar project, then you know how annoying it can be to manually copy across all the necessary files and then modify them to suit your new project.

Wouldn't it be nice if something automated this for you?

That's where Zat comes in:
- Create a Zat repository to represent the common files and folder structures across your project.
- Tokenise any differences that are project-specific.
- Use Zat to process the repository and generate a unique versions of the project at a new destination.

## Installation

### Downloading a Release

Download the latest [release](https://github.com/ssanj/zat/releases) for your operating system (linux or macos).
Make it executable with:

`chmod +x <ZAT_EXEC>`

Copy executable to a directory on your path.

Note: Windows is not explicitly supported at this time as I don't use Windows myself. If you install Zat on Windows and have any problems please raise an issue.


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
  process         Process templates defined in a Zat repository
  bootstrap       Generate a minimal bootstrap Zat repository
  process-remote  Process templates defined in a remote Zat repository
  help            Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

Use `zat --help` for additional usage information.

## Getting Started

To get started generate a `bootstrap` project and follow the instructions.

To create simple bootstrap project run:

```
zat bootstrap --repository-dir <ZAT_REPOSITORY>
```

The `ZAT_REPOSITORY` directory will have the following files:

```{.terminal .scrollx}
├── template
│   ├── $project__underscore$_config.conf
│   └── README.md.tmpl
└── .variables.zat-prompt
```

![Creating a Bootstrap Project](docs/images/zat-bootstrap.gif)

Once the repository is created, you can run it with:

```
zat process --repository-dir  <ZAT_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

Once the template is processed the `WHERE_TO_EXTRACT_THE_REPOSITORY` directory will have the following files:

```{.terminal .scrollx}
├── my_cool_project_config.conf
└── README.md
```

![Processing a Bootstrap Project](docs/images/zat-process-bootstrap.gif)

Have a look at the files in `ZAT_REPOSITORY` and how they they are different in `WHERE_TO_EXTRACT_THE_REPOSITORY` to get a feel for how Zat works. For more detailed information read the contents that follows.

The [example tests](https://github.com/ssanj/zat/tree/main/tests/examples) are a good source of some sample Zat repositories.

Additional templates can also be found in the [Example Templates](#example-templates) section.

## Repository Structure

- All Zat configuration files go in the root of the Zat repository. These include the '.variables.zat-prompt' configuration file and the 'shell-hook.zat-exec' shell hook file. The '.variables.zat-prompt' defines any tokens you want replaced. The values for these tokens will be requested from the user when the template is processed. The optional 'shell-hook.zat-exec' file should be an executable file (chmod +x). It will get invoked after the repository has been processed, with single argument of the target directory path. Use this file to handle any post-processing tasks.

- All templated files go in the 'templates' folder under the Zat repository folder. This can include regular files, files and folders with tokenised names and templates.

**Regular files**: Plain old files without any tokens in their name or in their content. These will get copied "as is" to the target directory when the repository is processed. **Note:**
 attributes of a file will not be copied. If you need some attributes maintained for a file, you can do that through a shell hook file.

**Files and folders with tokenised names**: Files and folders with tokens in their name but not in their content. Eg. '$project$_README.md'. These tokens will be replaced when the repository is processed and files and folders will be written to the target directory with the updated names.

**Templates**: Are files that end with a '.tmpl'. Eg. 'README.md.tmpl'. They can have tokens in their name and in their content. The tokens in their names and content will get replaced when the repository is processed. The '.tmpl' suffix is removed when the processed template is written to the target directory.

### Repository Types

There are two types of repositories supported by Zat:

- Local repositories
- Remote repositories

Local repositories can be processed by the `process` command. A local repository is processed and copied over to the specified target directory:

```
zat process --repository-dir <YOUR_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```


Remote repositories can be processed by the `process-remote` command. Remote repositories will be `Git` cloned locally and then run as per the local repository workflow. To use remote repositories you need to have `Git` installed and on your `PATH`. The repository URL should be `http(s)` and should not require a password for your `Git` user:

```
zat process-remote --repository-url <YOUR_REMOTE_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```


![Processing a Remote Project](docs/images/zat-process-remote.gif)

## Defining a Template

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


### Defining Tokens (or variables)

Define a file named `.variables.zat-prompt` in the root of your Zat repository. This file will determine the tokens that can be used within the project, either as variable substitutions for file and directory names or file content within a files with the `.tmpl` suffix (templates).

When you `process` the Zat repository, you will be prompted to enter the values of the defined variables.

[Minimal Json schema](dot-variables.schema.json)

Here's the structure of entries in the `variables.prompt` file:


| Field | What is it? |
| ----- | ---------- |
| variable_name | Name of the variable to define |
| description |  The description of the variable |
| prompt | What to show the user when prompting for this variable |
| filters | [Optional], The list of [filters](#structure-of-a-filter) available to the variable |
| default_value | [Optional] a default value to use, which the user can override if required |

#### Structure of a Filter

A filter is some type of [formatting](#supported-filters) that is applied to the user-supplied value. An example is `Pascal` filter, which converts the value into camel case with all the spaces removed:

`"My variable NAME" -> "MyVariableName"`

See [Supported Filters](#supported-filters) for a full list of supported formats.

| Field | What is it? |
| ----- | ---------- |
| name| Name of the filter. `__default__` is used for when this should be the default filter applied. A default filter will not have the filter name appended to the variable name at the usage point. It can simply use the variable name. Eg. $project$. |
| filter | one of the support filters (see below) |


#### Example Configuration

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
# $project$ <-- default filter for `project`, which applies the `Pascal` format.

$description$ <-- variable without filters

Folders will be like: $project__underscore$  <-- variable using the 'underscore' filter on the `project` value, which applies the `Snake` format.
```

Assuming the user supplied the following values for the variables:

|Variable| User-supplied Value|
|---|---|
|project| "My Cool Project"|
|description| [accepts default] |


##### Processed Template

```markdown
# MyCoolProject

Some project description

Folders will be like: my_cool_project
```

</details>

#### Supported Filters

Filters modify the user-supplied variables. You can think of a filter as a function from `String` -> `String`.

| Filter | Example |
| -----  | ---------- |
| Camel  |  "My variable NAME" -> "myVariableName"  |
| Cobol  | "My variable NAME" -> "MY-VARIABLE-NAME" |
| Flat   | "My variable NAME" -> "myvariablename"   |
| Kebab  | "My variable NAME" -> "my-variable-name" |
| Lower  | "My variable NAME" -> "my variable name" |
| Noop   | "My variable NAME" -> "My variable NAME" |
| Pascal | "My variable NAME" -> "MyVariableName"   |
| Snake  | "My variable NAME" -> "my_variable_name" |
| Title  | "My variable NAME" -> "My Variable Name" |
| Upper  | "My variable NAME" -> "MY VARIABLE NAME" |


#### Filter Usage

Variables can be used in the following ways:

- As a file name (eg. ` $project$.sublime-settings`)
- As a directory name (eg. `/models/$project$/scaffold.rs`)
- Within a file with a `.tmpl` extension (eg. `$project__python$.py.tmpl`)

A variable with a filter can be used in the following format:

`variable_name` + __ + `filter_name`.

Given a variable named "project" and a filter named "python", we can use it as:

`$project__python$.py.tmpl`

As noted above, if the one of the filter names is `__default__` you can simply use the `variable_name` without appending the filter name.

#### Ignore Usage

You can ignore specified files from your `templates` folder using a regular expression. By default any `^.git` files are ignored. You can specify multiple ignored files and directories as:

```
zat process --ignore '^folderX/.*' --ignore '^never.txt' ....
```

Ignored files will not be processed or copied over to the target directory.

### Example Templates

| Template | Description |
| ----- | ---------- |
| [Sublime Text Plugin Template](https://github.com/ssanj/st-plugin-zat) | Simple Sublime Text Plugin template |
| [Rust CLI Template](https://github.com/ssanj/rust-cli-zat) | A template for creating CLI applications in Rust |
| [Basic Scala 3 Template](https://github.com/ssanj/basic-scala3-zat) | A template for creating applications in Scala 3 |
| [Basic Scala 3 Template with the Latest Dependencies](https://github.com/ssanj/basic-scala3-latest-deps-zat) | A template for creating applications in Scala 3 with the latest stable dependencies |

Also have a look at the [example tests](https://github.com/ssanj/zat/tree/main/tests/examples) for some sample Zat repositories.

A simple way to run a remote template is with `process-remote`. For example to run the `Sublime Text Plugin Template` use:

```
zat process-remote --repository-url https://github.com/ssanj/st-plugin-zat --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

## Plugins

Zat supports the concept of a "plugin"; an external executable that can be passed some arguments and which returns a result in a standard format. The result will be used to replace the value of a token in a template file. The executable can be a Bash script, a Rust program or anything you want really.

### Inputs

The inputs to the plugin can take any format the plugin requires.


### Outputs

The output from a plugin should follow the guidelines below.

For a successful result, the plugin should return the following JSON payload:

```json
{
    "success":
    {
        "result": "YOUR_RESULT"
    }
}
```

For example:


```.json
{
    "success":
    {
        "result": "2.10.0"
    }
}
```

For an erroneous result, the plugin should return the following JSON payload:

```json
{
    "error":
    {
        "plugin_name": "<YOUR_PLUGIN_NAME>",
        "error": "<DESCRIPTION_OF_THE_ERROR>",
        "exception": "<EXCEPTION_IF_ANY_THIS_IS_OPTIONAL>",
        "fix": "<HOW TO FIX THE ERROR>"
    }
}
```

For example:

```json
{
    "error":
    {
        "plugin_name": "scala-deps",
        "error": "The 'scala-deps' plugin did not receive any matching results from coursier.",
        "fix": "Verify the output returned by courser by running 'cs complete-dep org.typelevel:cats-snore_2.13:'"
    }
}
```

### Calling the plugin

The plugin can be called through `.variables.zat-prompt` configuration using the `plugin` attribute when defining a `token`:

```json
  {
    "variable_name": "YOUR_VARIABLE_NAME",
    "description": "WHAT YOUR_VARIABLE_NAME IS",
    "prompt": "HOW TO ASK FOR YOUR_VARIABLE_NAME",
    "plugin": {
      "id": "<NAME OR PATH TO PLUGIN>",
      "args":[
          "ARG1",
          "ARG2",
          "ARGX",
      ]
    }
  }
```

For example:

```json
  {
    "variable_name": "scala_3_version",
    "description": "Which version of Scala 3 to use",
    "prompt": "Please enter Scala 3 version to use",
    "plugin": {
      "id": "scala-deps",
      "args":[
          "-o",
          "org.scala-lang",
          "-g",
          "scala3-library",
          "-s",
          "3"
      ]
    }
  }
```

The value returned from the plugin can be used in templates as per usual. For example to use the above `scala_3_version` in a template we would use:

```
// YOUR.tmpl.file
$scala_3_version$
```

If the plugin returns successfully the token will be replaced within the template file. If the plugin fails, the failure message will be displayed and Zat will fail.

### Examples

The simplest plugins are shell scripts.

#### Successful Plugin

Here's an example of the Bash plugin `success.sh` that echoes the parameter supplied to it:

```bash
#!/bin/bash

if [[ "$#" -ne 1 ]];then
  echo "usage: success.sh <MESSAGE>"
  exit 1
fi


MESSAGE="$1"
echo "{\"success\":{\"result\": \"$MESSAGE\"}}"
```

#### Failure Plugin

Here's an example of the Bash plugin `failure.sh` that echoes the parameter supplied to it as an error:

```bash
if [[ "$#" -ne 1 ]];then
  echo "usage: failure.sh <MESSAGE>"
  exit 1
fi

MESSAGE="$1"
echo {\"error\":{\"plugin_name\": \"failure.sh\",\"error\": \"$MESSAGE.\",\"exception\": \"Born to fail\",\"fix\": \"Run the success.sh plugin instead.\"}}
```

Have a look at the [scala-deps](https://github.com/ssanj/scala-deps-zatp) plugin for a working Rust implementation. Some other simple Bash examples can be found in the [plugin folder](https://github.com/ssanj/zat/tree/main/tests/plugins).

## Caveats

- Zat does not officially support Windows.
- Tokens defined in the `.variables.zat-prompt` file can't be escaped within a template file; The whole purpose of these tokens, is to get replaced within a template file.
- You need to have `Git` installed and on your `PATH` to use process remote repositories. Remote repositories should not need a password for your given `Git` user and should be `http(s)` URLs.
- File attributes are not copied over when a repository is processed. Use the `shell-hook.zat-exec` file to apply any attributes required in post processing.
