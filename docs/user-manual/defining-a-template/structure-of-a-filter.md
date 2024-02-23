# Structure of a Filter

A filter is some type of [formatting](filters/supported-filters.md) that is applied to the user-supplied value. An example is `Pascal` filter, which converts the value into camel case with all the spaces removed:

`"My variable NAME" -> "MyVariableName"`

See [Supported Filters](filters/supported-filters.md) for a full list of supported formats.

| Field | What is it? |
| ----- | ---------- |
| name| Name of the filter. `__default__` is used for when this should be the default filter applied. A default filter will not have the filter name appended to the variable name at the usage point. It can simply use the variable name. Eg. $project$. |
| filter | one of the support filters (see below) |

- [Supported filters](filters/supported-filters.md)
- [How to use a filter](filters/how-to-use-a-filter.md)
