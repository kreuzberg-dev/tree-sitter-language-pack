```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"typescript"}');
  final result = await TreeSitterLanguagePackBridge.process('const x: number = 42;', config: _config);
}

```
