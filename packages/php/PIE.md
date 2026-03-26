# PIE Installation

[PIE](https://github.com/php/pie) is the recommended way to install PHP extensions.

## Install

```bash
pie install kreuzberg/tree-sitter-language-pack
```

PIE will automatically download the pre-built binary for your platform and configure PHP.

## Supported Platforms

- Linux x86_64
- Linux aarch64 (ARM64)
- macOS ARM64 (Apple Silicon)

## Requirements

- PHP >= 8.2

## Build from Source

If PIE cannot find a pre-built binary for your platform, it will attempt to build from source. This requires:

- Rust toolchain (1.75+)
- C compiler (gcc/clang)
- PHP development headers (`php-dev` or `php-devel`)

```bash
# Install Rust if needed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Then install via PIE (will build from source)
pie install kreuzberg/tree-sitter-language-pack
```

## Verify Installation

```bash
php -m | grep ts_pack_php
php -r "echo ts_pack_version() . PHP_EOL;"
```

## Manual Installation

Download the pre-built extension from [GitHub Releases](https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases):

1. Extract the platform-specific tarball
2. Copy the extension file from `ext/` to your PHP extension directory
3. Add to your `php.ini`:

   ```ini
   extension=libts_pack_php.so
   ```

## Troubleshooting

**Extension not found after installation:**

```bash
# Find your PHP extension directory
php -i | grep extension_dir

# Verify the file exists
ls -la $(php -i | grep extension_dir | awk '{print $NF}')/libts_pack_php.*
```

**Version mismatch:**

```bash
# Check installed version
php -r "echo ts_pack_version();"
```
