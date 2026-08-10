```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"gosum"}');
  final result = await TreeSitterLanguagePackBridge.process('example.com/pkg v1.0.0 h1:abc=', config: _config);
}

```
