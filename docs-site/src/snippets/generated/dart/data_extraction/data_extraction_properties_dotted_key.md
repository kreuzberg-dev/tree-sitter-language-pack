```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"properties"}');
  final result = await TreeSitterLanguagePackBridge.process('server.host=localhost\nserver.port=8080\n', config: _config);
}

```
