# Wink

Wink is a collection of filesystem link tools for windows.

It only works on NTFS.

# What's Included

-	`link` command: Create symbolic links, hard links and junctions.
-	`rmlink` command: Safely remove symbolic links and junctions.
-	`linfo`: Test if files are symbolic links or junctions.

# Installation

You have 2 options:

### Install from crates.io

`cargo install wink`

### Install after git clone

```sh
git clone https://github.com/insomnimus/wink
cd wink
cargo install --path .
```

# Usage

The commands are very simple, please run them with `--help` for the usage for each.
