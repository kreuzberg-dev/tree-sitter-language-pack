```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"python"}');
  final result = await TreeSitterLanguagePackBridge.process('def greet(name):\n    return f\'Hello, {name}!\'\n', config: _config);
}

```
