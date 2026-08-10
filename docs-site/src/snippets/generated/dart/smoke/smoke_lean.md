```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"lean"}');
  final result = await TreeSitterLanguagePackBridge.process('def main : IO Unit := pure ()', config: _config);
}

```
