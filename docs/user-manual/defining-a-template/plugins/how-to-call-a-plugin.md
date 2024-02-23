# How to call a plugin

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
