```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("require 'json'\n\nclass Greeter\n  def greet(name)\n    \"Hello \#{name}\"\n  end\nend\n", { 'language' => 'ruby' })

```
