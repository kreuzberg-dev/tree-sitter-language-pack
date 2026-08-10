```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"editorconfig"}');
  final result = await TreeSitterLanguagePackBridge.process('[*.rs]\nindent_style = space\nindent_size = 4\n', config: _config);
}

```
