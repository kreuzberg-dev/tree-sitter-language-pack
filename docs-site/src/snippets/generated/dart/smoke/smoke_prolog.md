---
id: fixture_dart_smoke_prolog
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
    final _config = await createProcessConfigFromJson(json: '{"language":"prolog"}');
    final result = await TreeSitterLanguagePackBridge.process('hello :- write(\'hello\'), nl.', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
