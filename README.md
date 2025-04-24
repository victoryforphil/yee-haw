# Yee Haw

Smart file wrangler for the terminal that helps organize and deduplicate files across directories.

## Features

- Scan file trees using glob patterns to match files
- Calculate file hashes to detect and handle duplicates
- Organize files into groups based on their source directories
- Multiple file renaming styles for the destination
- Flexible folder grouping options
- Intelligent duplicate handling - duplicates stored in _dupes directory
- Dry run capability to preview actions without making changes

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/yee-haw.git
cd yee-haw

# Build the project
cargo build --release

# Run the binary
./target/release/yee-haw --help
```

## Usage

```bash
# Basic usage
yee-haw -s <SOURCE_DIR> -d <DESTINATION_DIR> -q "<GLOB_PATTERN>"

# Examples:
# Find and organize all JPG files from Photos to Organized directory
yee-haw -s ~/Photos -d ~/Organized -q "*.jpg"

# Organize MP3 files with lowercase renaming and incremental grouping
yee-haw -s ~/Music -d ~/Sorted -q "*.mp3" --rename-style lowercase --group-style incremental

# Dry run to preview what would happen
yee-haw -s ~/Documents -d ~/Backup -q "*.pdf" --dry

# Handle duplicates by moving them to a _dupes directory
yee-haw -s ~/Photos -d ~/Organized -q "*.jpg" --track-duplicates
```

## CLI Options

| Option | Description | Default |
|--------|-------------|---------|
| `-s, --source-dir` | Source directory to scan | `./` |
| `-q, --query` | Query (glob pattern) to match files | `*` |
| `-d, --destination-dir` | Destination directory to move files to | `./out` |
| `--dry` | Perform a dry run (don't actually move files) | `false` |
| `--track-duplicates` | Track and handle duplicates separately | `true` |
| `--rename-style` | File renaming style for destination | `none` |
| `--group-style` | Grouping style for destination folders | `short-hash` |
| `-h, --help` | Print help | |
| `-V, --version` | Print version | |

### Rename Styles

- `none`: Keep original filenames
- `lowercase`: Convert filenames to lowercase
- `incremental`: Add incremental numbers to filenames
- `short-hash`: Use short hash for filenames
- `combined`: Combine original name with a hash

### Group Styles

- `short-hash`: Use short hash for destination folder names
- `incremental`: Use incremental numbers for destination folder names

## Duplicate Handling

When duplicate files are detected (files with identical content):

1. The first encountered file is considered the "original" and is moved to the destination directory
2. Any duplicates are moved to a `_dupes` folder inside the source directory
3. The duplicates maintain the same directory structure they would have had in the destination

This allows you to easily identify and manage duplicate files while preserving their organizational context.

## Environment Variables

- `RUST_LOG`: Set logging level (`info`, `debug`, `trace`)
  ```bash
  RUST_LOG=debug yee-haw -s ~/Photos -d ~/Organized -q "*.jpg"
  ```

## License

This project is licensed under the MIT License - see the LICENSE file for details.