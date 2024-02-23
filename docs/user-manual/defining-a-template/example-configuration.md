# Example Configuration


## .variables.zat-prompt file:
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

## Template that uses the above variables:

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


## Processed Template

```markdown
# MyCoolProject

Some project description

Folders will be like: my_cool_project
```
