# Wink

Wink is a collection of filesystem link tools for windows.

Symbolic and hard links work on any filesystem but junctions are an NTFS feature so they most definitely won't.

## What's Included
- `ln`: Create symbolic links, hard links and NTFS junctions.
- `rmlink`: Safely remove a junction or a symbolic link; fail if not a link.
- `linfo`: Display if a file is a symbolic link or a junction.

## Installation
### From My Scoop Bucket
1. Add my [scoop bucket](https://github.com/insomnimus/scoop-bucket) to scoop:\
	`scoop bucket add insomnia https://github.com/insomnimus/scoop-bucket`
2. Update scoop:\
	`scoop update`
3. Install `wink`:\
	`scoop install wink`

### Compile It Yourself
```sh
cargo install --git https://github.com/insomnimus/wink --branch main
# Or, from crates.io:
# cargo install wink
```

## Usage
The commands are pretty straightforward; please run each with `--help` for the usage.
