```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"diagnostics":true,"language":"python"}');
  final result = await TreeSitterLanguagePackBridge.process('def broken(\n    pass\n', config: _config);
}

```
