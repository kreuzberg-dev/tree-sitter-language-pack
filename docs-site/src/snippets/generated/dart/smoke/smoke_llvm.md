```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"llvm"}');
  final result = await TreeSitterLanguagePackBridge.process('define i32 @main() { ret i32 0 }', config: _config);
}

```
