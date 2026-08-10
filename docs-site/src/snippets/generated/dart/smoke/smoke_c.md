```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"c"}');
  final result = await TreeSitterLanguagePackBridge.process('int main() { return 0; }', config: _config);
}

```
