```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"json"}');
  final result = await TreeSitterLanguagePackBridge.process('{"host": "localhost", "port": 8080}', config: _config);
}

```
