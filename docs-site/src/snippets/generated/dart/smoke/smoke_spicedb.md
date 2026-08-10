```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"spicedb"}');
  final result = await TreeSitterLanguagePackBridge.process('definition user {}\n', config: _config);
}

```
