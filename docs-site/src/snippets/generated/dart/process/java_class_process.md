```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"java"}');
  final result = await TreeSitterLanguagePackBridge.process('import java.util.List;\n\npublic class Greeter {\n    public String greet(String name) {\n        return "Hello " + name;\n    }\n}\n', config: _config);
}

```
