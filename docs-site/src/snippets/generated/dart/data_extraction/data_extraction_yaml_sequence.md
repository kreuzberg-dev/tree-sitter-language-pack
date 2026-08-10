```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"yaml"}');
  final result = await TreeSitterLanguagePackBridge.process('ports:\n  - 8080\n  - 8081\n', config: _config);
}

```
