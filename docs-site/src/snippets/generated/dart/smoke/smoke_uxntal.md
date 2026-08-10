```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"uxntal"}');
  final result = await TreeSitterLanguagePackBridge.process('|0100 LIT 01', config: _config);
}

```
