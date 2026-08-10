```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"gitattributes"}');
  final result = await TreeSitterLanguagePackBridge.process('*.txt text', config: _config);
}

```
