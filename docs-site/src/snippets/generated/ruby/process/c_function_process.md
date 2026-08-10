```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("\#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", { 'language' => 'c' })

```
