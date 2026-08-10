```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"python"}');
  final result = await TreeSitterLanguagePackBridge.process('# A comment\ndef greet(name):\n    """Say hello."""\n    return f\'Hi {name}\'\n\nimport os\n', config: _config);
}

```
