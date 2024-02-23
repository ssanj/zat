# Supported Filters

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
