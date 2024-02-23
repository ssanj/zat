# Getting Started

To get started generate a `bootstrap` project and follow the instructions.

To create simple bootstrap project run:

```
zat bootstrap --repository-dir <ZAT_REPOSITORY>
```

The `ZAT_REPOSITORY` directory will have the following files:

```{.terminal .scrollx}
├── template
│   ├── $project__underscore$_config.conf
│   └── README.md.tmpl
└── .variables.zat-prompt
```

![Creating a Bootstrap Project](docs/images/zat-bootstrap.gif)

Once the repository is created, you can run it with:

```
zat process --repository-dir  <ZAT_REPOSITORY> --target-dir <WHERE_TO_EXTRACT_THE_REPOSITORY>
```

Once the template is processed the `WHERE_TO_EXTRACT_THE_REPOSITORY` directory will have the following files:

```{.terminal .scrollx}
├── my_cool_project_config.conf
└── README.md
```

![Processing a Bootstrap Project](docs/images/zat-process-bootstrap.gif)

Have a look at the files in `ZAT_REPOSITORY` and how they they are different in `WHERE_TO_EXTRACT_THE_REPOSITORY` to get a feel for how Zat works. For more detailed information read the contents that follows.

The [example tests](https://github.com/ssanj/zat/tree/main/tests/examples) are a good source of some sample Zat repositories.

Additional templates can also be found in the [Example Templates](defining-a-template/example-templates.md) section.
