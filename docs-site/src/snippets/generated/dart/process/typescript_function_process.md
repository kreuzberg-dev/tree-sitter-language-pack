```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"typescript"}');
  final result = await TreeSitterLanguagePackBridge.process('import { readFile } from \'fs\';\n\nfunction greet(name: string): string {\n    return `Hello, \${name}!`;\n}\n', config: _config);
}

```
