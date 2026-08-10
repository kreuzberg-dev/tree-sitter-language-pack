```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"python"}');
  final result = await TreeSitterLanguagePackBridge.process('class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def subtract(self, a, b):\n        return a - b\n', config: _config);
}

```
