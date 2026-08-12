---
id: fixture_dart_ruby_class_process
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
    final _config = await createProcessConfigFromJson(json: '{"language":"ruby"}');
    final result = await TreeSitterLanguagePackBridge.process('require \'json\'\n\nclass Greeter\n  def greet(name)\n    "Hello #{name}"\n  end\nend\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
