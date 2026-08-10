```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"javascript"}');
  final result = await TreeSitterLanguagePackBridge.process('export function greet(name) {\n  return `Hello \${name}`;\n}\n\nexport const VERSION = \'1.0\';\n', config: _config);
}

```
