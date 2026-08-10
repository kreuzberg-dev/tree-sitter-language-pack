```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"python"}');
  final result = await TreeSitterLanguagePackBridge.process('import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n', config: _config);
}

```
