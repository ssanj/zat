# Scopes

Scopes allow you to choose when a particular variable or plugin is required. Scopes depend on [choices](choices.md).


## Types of Scopes

There are four different types of scopes.

### Include by choice

These scopes will include variables only when a particular choice has been taken (with any value). The variable will not be displayed if the choice has not been made.

For example, to only ask for a `summary`, when `readme_type` has been chosen:

```
[
  {
    "variable_name":"summary",
    "description": "summary for longer README",
    "prompt": "Please choose your summary",
    "scopes": [
      {
        "choice": "readme_type"
      }
    ]
  }
]
```

### Include by choice and value

These scopes will include variables only when a particular choice has been taken with a specific value. The variable will not be displayed if the choice has not been made with that specific value.

For example, to only ask for a `summary`, when `readme_type` of `long` has been chosen:

```
[
  {
    "variable_name":"summary",
    "description": "summary for longer README",
    "prompt": "Please choose your summary",
    "scopes": [
      {
        "choice": "readme_type",
        "value": "long"
      }
    ]
  }
]
```


### Exclude by choice

These scopes will exclude variables only when a particular choice has been taken (with any value). The variable will be displayed if the choice has not been made (since this is an exclusion).

For example, to not ask for a `summary`, when `readme_type` has been chosen:

```
[
  {
    "variable_name":"summary",
    "description": "summary for longer README",
    "prompt": "Please choose your summary",
    "scopes": [
      {
        "not_choice": "readme_type"
      }
    ]
  }
]
```


### Exclude by choice and value

These scopes will exclude variables only when a particular choice has been taken with a specific value. The variable will be displayed if the choice has not been made or the choice has been made with a different value.

For example, to not ask for a `summary`, when a `readme_type` of `short` has been chosen:

```
[
  {
    "variable_name":"summary",
    "description": "summary for longer README",
    "prompt": "Please choose your summary",
    "scopes": [
      {
        "choice": "readme_type",
        "not_value": "short"
      }
    ]
  }
]
```


## Defining Scopes

Here's a full example of asking the user to make a choice on a type of README and then asking for a summary value only if the chosen type is a `long`:

```
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
    "variable_name": "readme_type",
    "description": "Type of README",
    "prompt": "Please choose your type of README",
    "choices": [
      {
        "display": "Short",
        "description": "A shorter README",
        "value": "short"
      },
      {
        "display": "Long",
        "description": "A longer README",
        "value": "long"
      }
    ]
  },
  {
    "variable_name":"summary",
    "description": "summary for longer README",
    "prompt": "Please choose your summary",
    "scopes": [
      {
        "choice": "readme_type",
        "value": "long"
      }
    ]
  }
]
```
