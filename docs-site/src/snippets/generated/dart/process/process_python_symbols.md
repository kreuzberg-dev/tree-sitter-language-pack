---
id: fixture_dart_process_python_symbols
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
    final _config = await createProcessConfigFromJson(json: '{"language":"python","symbols":true}');
    final result = await TreeSitterLanguagePackBridge.process('MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
