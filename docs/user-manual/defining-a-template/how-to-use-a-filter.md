# Filter Usage

Variables can be used in the following ways:

- As a file name (eg. ` $project$.sublime-settings`)
- As a directory name (eg. `/models/$project$/scaffold.rs`)
- Within a file with a `.tmpl` extension (eg. `$project__python$.py.tmpl`)

A variable with a filter can be used in the following format:

`variable_name` + __ + `filter_name`.

Given a variable named "project" and a filter named "python", we can use it as:

`$project__python$.py.tmpl`

As noted above, if the one of the filter names is `__default__` you can simply use the `variable_name` without appending the filter name.
