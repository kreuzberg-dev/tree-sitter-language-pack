---
id: fixture_dart_java_class_process
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
import 'package:tree_sitter_language_pack/src/tree_sitter_language_pack_bridge_generated/frb_generated.dart' show RustLib;
Future<void> main() async {
  await RustLib.init();
  try {
    final _config = await createProcessConfigFromJson(json: '{"language":"java"}');
    final result = await TreeSitterLanguagePackBridge.process('import java.util.List;\n\npublic class Greeter {\n    public String greet(String name) {\n        return "Hello " + name;\n    }\n}\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
