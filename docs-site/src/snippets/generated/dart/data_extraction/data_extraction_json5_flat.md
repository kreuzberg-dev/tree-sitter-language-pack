```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"json5"}');
  final result = await TreeSitterLanguagePackBridge.process('{\n  host: "localhost",\n  port: 8080,\n}\n', config: _config);
}

```
