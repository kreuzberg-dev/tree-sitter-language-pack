```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"rust"}');
  final result = await TreeSitterLanguagePackBridge.process('struct Point { x: f64, y: f64 }', config: _config);
}

```
