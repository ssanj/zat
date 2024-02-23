# Installation

## Downloading a Release

Download the latest [release](https://github.com/ssanj/zat/releases) for your operating system (linux or macos).
Make it executable with:

`chmod +x <ZAT_EXEC>`

Copy executable to a directory on your path.

Note: Windows is not explicitly supported at this time as I don't use Windows myself. If you install Zat on Windows and have any problems please raise an issue.


## Building through Cargo

You can build Zat through Cargo with:

```
cargo install --git https://github.com/ssanj/zat
```

This will install Zat into your Cargo home directory; usually `~/.cargo/bin`.

## Building from Source

Ensure you have Cargo installed.

Run:

```
cargo build --release
Copy binary file from target/release/zat to a directory on your PATH.
```
