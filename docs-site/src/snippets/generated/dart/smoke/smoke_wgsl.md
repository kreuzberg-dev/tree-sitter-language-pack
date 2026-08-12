---
id: fixture_dart_smoke_wgsl
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
    final _config = await createProcessConfigFromJson(json: '{"language":"wgsl"}');
    final result = await TreeSitterLanguagePackBridge.process('@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
