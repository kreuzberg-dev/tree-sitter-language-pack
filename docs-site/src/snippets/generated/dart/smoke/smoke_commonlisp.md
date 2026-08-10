```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"commonlisp"}');
  final result = await TreeSitterLanguagePackBridge.process('(defun hello () (print "hello"))', config: _config);
}

```
