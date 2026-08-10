```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("<!ELEMENT server (host, port)>\n<!ELEMENT host (\#PCDATA)>\n", { 'data_extraction' => true, 'language' => 'dtd' })

```
