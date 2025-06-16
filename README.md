# week

[![Build & Test](https://github.com/billyto/week/actions/workflows/build.yml/badge.svg)](https://github.com/billyto/week/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple and fast CLI utility to get ISO week numbers (1-53) for any date.

## 🚀 Quick Start

```bash
# Get current week number
week

# Get week number for a specific date
week --date 19-02-2023
week -d 15/03/2024
week -d 01.01
```

## 📖 Usage

```
week [OPTIONS]

OPTIONS:
    -d, --date <DATE>    Date to get week number for (various formats supported)
    -h, --help          Print help information
    -V, --version       Print version information
```

### Examples

```bash
# Current week
$ week
Week 24

# Specific date with year
$ week --date 19-02-2023
Week 7

$ week -d 25/12/2023
Week 52

# Date without year (uses current year)
$ week -d 15/03
Week 11

$ week -d 01.01
Week 1
```

## 📅 Supported Date Formats

The tool accepts dates in multiple convenient formats:

| Format       | Example      | Description                   |
|--------------|--------------|-------------------------------|
| `DD-MM-YYYY` | `19-02-2023` | Day-Month-Year with dashes    |
| `DD/MM/YYYY` | `19/02/2023` | Day-Month-Year with slashes   |
| `DD.MM.YYYY` | `19.02.2023` | Day-Month-Year with periods   |
| `DD-MM`      | `19-02`      | Day-Month (uses current year) |
| `DD/MM`      | `19/02`      | Day-Month (uses current year) |
| `DD.MM`      | `19.02`      | Day-Month (uses current year) |

> **Note**: All formats use DD-MM ordering (day first, then month)

## 🛠️ Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/billyto/week.git
cd week

# Build and install
cargo install --path week
```

### From Releases

Download pre-built binaries from the [releases page](https://github.com/billyto/week/releases) for:
- Windows (x86_64)
- Linux (x86_64)
- macOS (x86_64)

## 🧪 Development

### Prerequisites

- Rust 1.70.0 or later
- Cargo

### Building

```bash
cd week
cargo build --release
```

### Running Tests

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test cli
```

### Project Structure

```
week/
├── src/
│   ├── lib.rs          # Core date parsing logic
│   └── main.rs         # CLI interface
├── tests/
│   └── cli_tests.rs    # Integration tests
└── Cargo.toml          # Dependencies and metadata
```

## 📋 About ISO Week Numbers

This tool returns ISO week numbers according to [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):

- Week numbers range from 1 to 53
- Week 1 is the first week with at least 4 days in the new year
- Weeks start on Monday
- Some years have 53 weeks


## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🗺️ Roadmap

- [x] Support multiple date formats
- [x] CI/CD pipeline for releases
- [x] Comprehensive CLI help
- [x] Integration tests
- [ ] Add more date format support (ISO 8601, US format)
- [ ] Verbose and Quiet flags
- [ ] Cowsay option 🐮

## 🐛 Known Issues

- Error handling for invalid dates could be more descriptive
- Limited to Gregorian calendar only

