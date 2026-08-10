```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"python","symbols":true}');
  final result = await TreeSitterLanguagePackBridge.process('MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n', config: _config);
}

```
