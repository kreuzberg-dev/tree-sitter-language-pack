```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"comments":true,"language":"python"}');
  final result = await TreeSitterLanguagePackBridge.process('# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n', config: _config);
}

```
