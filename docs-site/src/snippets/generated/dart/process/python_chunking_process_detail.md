```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"chunk_max_size":30,"language":"python"}');
  final result = await TreeSitterLanguagePackBridge.process('def alpha():\n    pass\n\ndef beta():\n    pass\n\ndef gamma():\n    pass\n\ndef delta():\n    pass\n', config: _config);
}

```
