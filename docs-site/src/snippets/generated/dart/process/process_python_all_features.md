```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"comments":true,"docstrings":true,"imports":true,"language":"python","structure":true,"symbols":true}');
  final result = await TreeSitterLanguagePackBridge.process('import os\nfrom pathlib import Path\n\n# Configuration\nMY_CONST = 42\n\ndef process_file(path):\n    """Process a file and return contents."""\n    with open(path) as f:\n        return f.read()\n\nclass FileProcessor:\n    def __init__(self, base_dir):\n        self.base_dir = base_dir\n', config: _config);
}

```
