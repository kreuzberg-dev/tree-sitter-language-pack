```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"wgsl"}');
  final result = await TreeSitterLanguagePackBridge.process('@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }', config: _config);
}

```
