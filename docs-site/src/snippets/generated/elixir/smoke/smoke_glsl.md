```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "glsl"}
result = TreeSitterLanguagePack.process("void main() { gl_Position = vec4(0.0); }", config_value)

```
