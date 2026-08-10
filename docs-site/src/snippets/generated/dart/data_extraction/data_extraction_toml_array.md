```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"toml"}');
  final result = await TreeSitterLanguagePackBridge.process('ports = [8080, 8081, 8082]\n', config: _config);
}

```
