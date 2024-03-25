# Scopes

Scopes allow you to choose when a particular variable or plugin is required. Scopes depend on [choices](choices.md).


## Types of Scopes

There are four different types of scopes.

### Include by choice

These scopes will include variables only when a particular choice has been taken (with any value). The variable will not be used if the choice has not been made.

For example, to only ask for a `summary`, when `readme_type` has been chosen:

```json
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

These scopes will include variables only when a particular choice has been taken with a specific value. The variable will not be used if the choice has not been made with that specific value.

For example, to only ask for a `summary`, when `readme_type` of `long` has been chosen:

```json
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

These scopes will exclude variables only when a particular choice has been taken (with any value). The variable will be used if the choice has not been made (since this is an exclusion).

For example, to not use `summary`, when `readme_type` has been chosen:

```json
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

These scopes will exclude variables only when a particular choice has been taken with a specific value. The variable will be used if the choice has not been made or the choice has been made with a different value.

For example, to not ask for a `summary`, when a `readme_type` of `short` has been chosen:

```json
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

With a `readme_type` of `short` the `summary` variable is not requested:

![Creating a Bootstrap Project](../../images/zat-scopes-1.gif)

With a `readme_type` of `long` the `summary` variable is requested:

![Creating a Bootstrap Project](../../images/zat-scopes-2.gif)

## Scope resolution

When you define multiple scopes for a single variable, the following rules apply. In general, the scopes are applied in an `or` relationship (at least one of the scopes should apply).

## Multiple includes

If you have multiple `include` scopes, if any one of them is valid, then the variable is requested. For the following example, if either `choice_1` or `choice_2` is defined, then `sample_var` is requested:

```json
[
  {
    "variable_name":"sample_var",
    "description": "sample variable",
    "prompt": "Please enter your sample variable value",
    "scopes": [
      {
        "choice": "choice_1",
        "choice": "choice_2",
      }
    ]
  }
]
```

## Multiple includes and excludes

If you have multiple `include` and `exclude` scopes, if any one of the `include`s or `exclude`s is valid, then the variable is requested.

For the following example, if `choice_1` is defined, then `sample_var` is requested:

```json
[
  {
    "variable_name":"sample_var",
    "description": "sample variable",
    "prompt": "Please enter your sample variable value",
    "scopes": [
      {
        "choice": "choice_1",
        "not_choice": "choice_2",
        "not_choice": "choice_3",
      }
    ]
  }
]
```

In the above example, if `choice_1` is not defined *and* `choice_2` or `choice_3` is not defined then `sample_var` is also requested.


## Multiple excludes

If you have multiple `exclude` scopes, if any one of them is valid, then the variable is requested. For the following example, if either `choice_1` or `choice_2` is not defined, then `sample_var` is requested:

```json
[
  {
    "variable_name":"sample_var",
    "description": "sample variable",
    "prompt": "Please enter your sample variable value",
    "scopes": [
      {
        "not_choice": "choice_1",
        "not_choice": "choice_2",
      }
    ]
  }
]
```
