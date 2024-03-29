# Example Templates

| Template | Description |
| ----- | ---------- |
| [Sublime Text Plugin Template](https://github.com/ssanj/st-plugin-zat) | Simple Sublime Text Plugin template |
| [Rust CLI Template](https://github.com/ssanj/rust-cli-zat) | A template for creating CLI applications in Rust |
| [Basic Scala 3 Template](https://github.com/ssanj/basic-scala3-zat) | A template for creating applications in Scala 3 |
| [Basic Scala 3 Template with the Latest Dependencies](https://github.com/ssanj/basic-scala3-latest-deps-zat) | A template for creating applications in Scala 3 with the latest stable dependencies |
| [Scala 2 Template with the Latest Dependencies and Choice of Test Framework](https://github.com/ssanj/scala-2-multiple-test-frameworks-zat) | A template for creating applications in Scala 2 with the latest stable dependencies and a choice of testing frameworks |

Also have a look at the [example tests](https://github.com/ssanj/zat/tree/main/tests/examples) for some sample Zat repositories.

A simple way to run a remote template is with `process-remote`. For example to run the `Sublime Text Plugin Template` use:

```
zat process-remote --repository-url https://github.com/ssanj/st-plugin-zat --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

![Processing a Remote Project](../../images/zat-process-remote.gif)
