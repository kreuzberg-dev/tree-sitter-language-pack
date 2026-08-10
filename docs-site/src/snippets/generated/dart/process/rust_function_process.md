```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"rust"}');
  final result = await TreeSitterLanguagePackBridge.process('fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n', config: _config);
}

```
