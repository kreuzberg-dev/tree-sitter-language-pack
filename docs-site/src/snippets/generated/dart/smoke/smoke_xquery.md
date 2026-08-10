```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"xquery"}');
  final result = await TreeSitterLanguagePackBridge.process('1\n', config: _config);
}

```
