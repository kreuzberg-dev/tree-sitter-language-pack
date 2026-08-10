```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"wdl"}');
  final result = await TreeSitterLanguagePackBridge.process('version 1.0\n', config: _config);
}

```
