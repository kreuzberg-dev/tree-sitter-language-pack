```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"abnf"}');
  final result = await TreeSitterLanguagePackBridge.process('a = "b"\r\n', config: _config);
}

```
