# Plugins

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

- [How to call a plugin](../plugins/how-to-call-a-plugin.md)
- [Example plugins](../plugins/example-plugins.md)

