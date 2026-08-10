```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"sas"}');
  final result = await TreeSitterLanguagePackBridge.process('data _null_;\nrun;\n', config: _config);
}

```
