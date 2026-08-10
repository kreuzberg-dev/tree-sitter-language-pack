```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "typescript"}
result = TreeSitterLanguagePack.process("import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, ${name}!`;\n}\n", config_value)

```
