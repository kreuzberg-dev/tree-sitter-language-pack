```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"glsl"}');
  final result = await TreeSitterLanguagePackBridge.process('void main() { gl_Position = vec4(0.0); }', config: _config);
}

```
