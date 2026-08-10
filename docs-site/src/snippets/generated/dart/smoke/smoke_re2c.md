```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"re2c"}');
  final result = await TreeSitterLanguagePackBridge.process('/*!re2c\n  [a-z]+ { return; }\n*/', config: _config);
}

```
