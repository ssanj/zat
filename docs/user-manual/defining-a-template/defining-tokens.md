# Defining Tokens (or variables)

Define a file named `.variables.zat-prompt` in the root of your Zat repository. This file will determine the tokens that can be used within the project, either as variable substitutions for file and directory names or file content within a files with the `.tmpl` suffix (templates).

When you `process` the Zat repository, you will be prompted to enter the values of the defined variables.

[Minimal Json schema](/dot-variables.schema.json)

Here's the structure of entries in the `variables.prompt` file:


| Field | What is it? |
| ----- | ---------- |
| variable_name | Name of the variable to define |
| description |  The description of the variable |
| prompt | What to show the user when prompting for this variable |
| filters | [Optional], The list of [filters](structure-of-a-filter.md) available to the variable |
| default_value | [Optional] a default value to use, which the user can override if required |
