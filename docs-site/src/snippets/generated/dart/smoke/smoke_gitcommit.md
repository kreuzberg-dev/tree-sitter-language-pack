```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"gitcommit"}');
  final result = await TreeSitterLanguagePackBridge.process('feat: add feature\n\nBody text', config: _config);
}

```
