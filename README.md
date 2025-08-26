# pwgen-vs-words

A Rust command-line tool that generates passwords using `pwgen` and checks if the generated passwords exist in a dictionary of common words. This is useful for security testing to identify weak passwords that might be vulnerable because they match dictionary words.

System/OS|Build Result
----:|----
Travis / Unix       |[![Build Status](https://travis-ci.org/zeljic/pwgen-vs-words.svg?branch=develop)](https://travis-ci.org/zeljic/pwgen-vs-words)
AppVeyor / Windows  |[![Build status](https://ci.appveyor.com/api/projects/status/aarood0ixbjscxqo/branch/develop?svg=true)](https://ci.appveyor.com/project/zeljic/pwgen-vs-words/branch/develop)

## Features

- Generate passwords using the system `pwgen` tool and filter them against dictionary words
- Check piped input passwords against dictionary words
- Configurable password length and generation count
- Parallel processing for improved performance
- Support for custom word dictionaries

## Installation

### Prerequisites

1. **Rust**: Install Rust from [rustup.rs](https://rustup.rs/)
2. **pwgen**: Install the pwgen password generation tool:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install pwgen
   
   # macOS
   brew install pwgen
   
   # Fedora/RHEL
   sudo dnf install pwgen
   ```

### Building from Source

```bash
git clone https://github.com/zeljic/pwgen-vs-words.git
cd pwgen-vs-words
cargo build --release
```

The compiled binary will be available at `target/release/pwgen-vs-words`.

## Usage

### Basic Syntax

```bash
pwgen-vs-words --letters <LENGTH> --dictionary <DICT_FILE> [OPTIONS]
```

### Required Arguments

- `-l, --letters <LENGTH>`: Length of passwords to generate/check
- `-d, --dictionary <DICT_FILE>`: Path to dictionary file (one word per line)

### Optional Arguments

- `-c, --count <COUNT>`: Number of passwords to generate (cannot be used with --pipe)
- `-p, --pipe`: Read passwords from standard input instead of generating them (cannot be used with --count)

### Examples

#### Generate passwords and check against dictionary

Generate 10 passwords of length 8 and show only those that match dictionary words:

```bash
pwgen-vs-words --letters 8 --dictionary /usr/share/dict/words --count 10
```

#### Check existing passwords from a file

Check if passwords in a file match dictionary words:

```bash
cat passwords.txt | pwgen-vs-words --letters 8 --dictionary /usr/share/dict/words --pipe
```

#### Using a custom dictionary

Create your own dictionary file:

```bash
echo -e "password\n123456\nletmein\nwelcome" > custom-dict.txt
pwgen-vs-words --letters 7 --dictionary custom-dict.txt --count 5
```

### Output

The tool will output only the generated/input passwords that are found in the dictionary, one per line. If no matches are found, no output is produced.

## Use Cases

- **Security Testing**: Identify weak passwords that match common dictionary words
- **Password Policy Validation**: Check if generated passwords comply with "no dictionary words" policies  
- **Penetration Testing**: Find potentially weak passwords in password lists
- **Educational**: Demonstrate why dictionary-based passwords are insecure

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests: `cargo test`
5. Run formatting: `cargo fmt`
6. Commit your changes (`git commit -m 'Add some amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

- **Đorđe Zeljić** - [zeljic](https://github.com/zeljic)
