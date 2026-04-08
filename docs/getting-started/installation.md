---
title: Installation
description: "Install tree-sitter-language-pack in Python, Node.js, Rust, Go, Java, C#, Ruby, Elixir, PHP, WebAssembly, or via the CLI."
---

tree-sitter-language-pack is available for every major ecosystem. All packages share the same version and API surface.

## Python

Requires Python 3.10+.

=== "pip"

    ```bash
    pip install tree-sitter-language-pack
    ```

=== "uv"

    ```bash
    uv add tree-sitter-language-pack
    ```

=== "poetry"

    ```bash
    poetry add tree-sitter-language-pack
    ```

Verify the installation:

```python
import tree_sitter_language_pack as tslp
print(tslp.language_count())  # 248
```

## Node.js

Requires Node.js 18+.

=== "npm"

    ```bash
    npm install @kreuzberg/tree-sitter-language-pack
    ```

=== "pnpm"

    ```bash
    pnpm add @kreuzberg/tree-sitter-language-pack
    ```

=== "yarn"

    ```bash
    yarn add @kreuzberg/tree-sitter-language-pack
    ```

Verify:

```javascript
const tslp = require("@kreuzberg/tree-sitter-language-pack");
console.log(tslp.languageCount()); // 248
```

The package ships pre-built native binaries for Linux (x64, arm64), macOS (x64, arm64), and Windows (x64).

## Rust

Requires Rust 1.75+.

```bash
cargo add ts-pack-core
```

Or add to `Cargo.toml` manually:

```toml
[dependencies]
ts-pack-core = "1"
```

Verify:

```rust
fn main() {
    println!("{}", ts_pack_core::language_count()); // 248
}
```

## Go

Requires Go 1.26+.

```bash
go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go/v1
```

```go
import tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go/v1"

func main() {
    fmt.Println(tslp.LanguageCount()) // 248
}
```

The Go binding uses cgo and links against the pre-compiled C FFI library.

## Java

Requires JDK 25+ (uses Panama FFM API).

=== "Maven"

    ```xml
    <dependency>
        <groupId>dev.kreuzberg</groupId>
        <artifactId>tree-sitter-language-pack</artifactId>
        <version>1.3.0</version>
    </dependency>
    ```

=== "Gradle (Kotlin)"

    ```kotlin
    dependencies {
        implementation("dev.kreuzberg:tree-sitter-language-pack:1.3.0")
    }
    ```

=== "Gradle (Groovy)"

    ```groovy
    dependencies {
        implementation 'dev.kreuzberg:tree-sitter-language-pack:1.3.0'
    }
    ```

```java
import dev.kreuzberg.TreeSitterLanguagePack;

public class Main {
    public static void main(String[] args) {
        System.out.println(TreeSitterLanguagePack.languageCount()); // 248
    }
}
```

## C# / .NET

Requires .NET 10+.

=== "dotnet CLI"

    ```bash
    dotnet add package TreeSitterLanguagePack
    ```

=== "Package Manager"

    ```powershell
    Install-Package TreeSitterLanguagePack
    ```

=== ".csproj"

    ```xml
    <PackageReference Include="TreeSitterLanguagePack" Version="1.3.0" />
    ```

```csharp
using TreeSitterLanguagePack;

Console.WriteLine(TsPackClient.LanguageCount()); // 248
```

## Ruby

Requires Ruby 3.4+.

=== "gem"

    ```bash
    gem install tree_sitter_language_pack
    ```

=== "Gemfile"

    ```ruby
    gem "tree_sitter_language_pack", "~> 1.0"
    ```

    ```bash
    bundle install
    ```

```ruby
require "tree_sitter_language_pack"

puts TreeSitterLanguagePack.language_count # 248
```

## Elixir

Requires Elixir 1.14+ and OTP 25+.

=== "mix.exs"

    ```elixir
    defp deps do
      [
        {:tree_sitter_language_pack, "~> 1.0"}
      ]
    end
    ```

    ```bash
    mix deps.get
    ```

```elixir
IO.puts TreeSitterLanguagePack.language_count() # 248
```

## PHP

Requires PHP 8.2+.

=== "Composer"

    ```bash
    composer require kreuzberg/tree-sitter-language-pack
    ```

=== "composer.json"

    ```json
    {
        "require": {
            "kreuzberg/tree-sitter-language-pack": "^1.0"
        }
    }
    ```

```php
<?php
echo \ts_pack_language_count(); // 248
```

## WebAssembly

Use from any JavaScript environment including browsers, Deno, and Cloudflare Workers.

=== "npm"

    ```bash
    npm install @kreuzberg/tree-sitter-language-pack-wasm
    ```

=== "CDN (browser)"

    ```html
    <script type="module">
      import { availableLanguages, parseString } from "https://cdn.jsdelivr.net/npm/@kreuzberg/tree-sitter-language-pack-wasm/+esm";
      console.log(availableLanguages());
    </script>
    ```

=== "Deno"

    ```typescript
    import { availableLanguages, parseString } from "npm:@kreuzberg/tree-sitter-language-pack-wasm";
    console.log(availableLanguages());
    ```

## CLI

The `ts-pack` binary provides parser management and code analysis from the terminal.

=== "Homebrew (macOS / Linux)"

    ```bash
    brew install kreuzberg-dev/tap/ts-pack
    ```

=== "Cargo"

    ```bash
    cargo install ts-pack-cli
    ```

Verify:

```bash
ts-pack --version
ts-pack list | wc -l  # 248
```

## Next Steps

- [Quick Start guide](quickstart.md) — parse your first file in 5 minutes
- [Download model](../concepts/download-model.md) — understand how parser caching works
- [Languages](../languages.md) — full list of 248 supported languages
