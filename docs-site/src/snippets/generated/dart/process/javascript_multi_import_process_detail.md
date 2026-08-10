```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"javascript"}');
  final result = await TreeSitterLanguagePackBridge.process('import fs from \'fs\';\nimport path from \'path\';\n\nfunction process(input) {\n    return input.trim();\n}\n', config: _config);
}

```
