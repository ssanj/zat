# Rust CLI Template

Defines a basic [Zat](https://github.com/ssanj/zat) repository for a Rust CLI project.

## Installation

```
zat process-remote --repository-url https://github.com/ssanj/rust-cli-zat  --target-dir <YOUR_TARGET_DIRECTORY>
```

## Functionality

- A simple [clap](https://docs.rs/clap/latest/clap) command line argument that accepts a verbosity (along with help and version).
- A `main` function that uses the above arguments.
- A integration test that invokes the program and verifies its version.


## Dependencies:
|Crate| Description | Features |
|-|-|-|
| [walkdir](https://docs.rs/walkdir/latest/walkdir/) | Directory traverser | `{ version = "2" }` |
| [serde](https://docs.rs/serde/latest/serde/) | serializer/deserializer | `{ version = "1", features = ["derive"]}` |
| [serde_json](https://docs.rs/serde_json/latest/serde_json) | JSON serializer/deserializer | `{ version = "1"}` |
| [clap](https://docs.rs/clap/latest/clap) | Commandline argument parser | `{ version = "4", features = ["derive", "cargo", "env", "unicode"] }` |
| [ansi_term](https://docs.rs/ansi_term/latest/ansi_term/) | ANSI terminal colours | `{ version = "0.12"}` |
| [dirs](https://docs.rs/dirs/latest/dirs) | Access to common directories | `{ version = "5"}` |


## Dev dependencies:

|Crate| Description | Features |
|-|-|-|
| [pretty_assertions](https://docs.rs/pretty_assertions/latest/pretty_assertions) | Better test assertions | `{ version = "1" }` |
| [assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd) | Easier command testing | `{ version = "2" }`|
| [dir-diff](https://docs.rs/dir_diff/latest/dir_diff) | Directory diffing | `{ version = "0.3" }` |
| [similar](https://docs.rs/similar/latest/similar) | Nicer diff output for changes | `{ version = "2" }`|
| [predicates](https://docs.rs/predicates/latest/predicates) | Additional assertions | `{ version = "3"}` |


## Scripts

Requires the following programs:
- [cargo-watch](https://crates.io/crates/cargo-watch) (To watch for file changes)
- [quiet](https://github.com/ssanj/quiet/) (To limit compiler output to a given number of errors)

|Script|Description|
|-|-|
|`compile`| compile code, while watching for changes |
|`compile-test`| compile test code, while watching for changes |
|`run-test`| run all tests, while watching for changes |
|`qcompile`| compile code, while watching for changes and limiting output |
|`qcompile-test`| compile test code, while watching for changes and limiting output |
|`qrun-test`| run all tests, while watching for chnges and limiting output |
|`release-local`| build a local release and copy it to `~/bin` |

## GitHub Actions

- Linux build (`build-linux.yml`) and release (`release-linux.yml`) for Intel CPUs on Ubuntu latest.
- macOS build (`build-mac.yml`) and release (`release-mac.yml`) for Apple Silicon and Intel CPU for macOS 12 and 13.

Note: A new release will be created every time a tag of type `vX.Y.Z` is pushed.

