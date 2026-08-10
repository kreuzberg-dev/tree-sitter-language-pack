```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"hocon"}');
  final result = await TreeSitterLanguagePackBridge.process('host = "localhost"\nport = 8080\n', config: _config);
}

```
