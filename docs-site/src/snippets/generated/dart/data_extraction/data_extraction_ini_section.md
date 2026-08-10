```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"ini"}');
  final result = await TreeSitterLanguagePackBridge.process('[database]\nhost=localhost\nport=5432\n', config: _config);
}

```
