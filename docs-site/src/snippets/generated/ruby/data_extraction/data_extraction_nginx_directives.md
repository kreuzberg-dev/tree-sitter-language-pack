```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", { 'data_extraction' => true, 'language' => 'nginx' })

```
