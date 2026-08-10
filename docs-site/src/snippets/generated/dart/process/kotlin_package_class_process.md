```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"kotlin"}');
  final result = await TreeSitterLanguagePackBridge.process('package foo.bar\n\nclass Widget {\n    fun greet(): String = "hi"\n}\n', config: _config);
}

```
