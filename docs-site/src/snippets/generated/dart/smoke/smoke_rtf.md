```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"rtf"}');
  final result = await TreeSitterLanguagePackBridge.process('{\\rtf1 hello}', config: _config);
}

```
