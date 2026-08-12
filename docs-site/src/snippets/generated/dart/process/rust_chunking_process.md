---
id: fixture_dart_rust_chunking_process
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
    final _config = await createProcessConfigFromJson(json: '{"chunk_max_size":30,"language":"rust"}');
    final result = await TreeSitterLanguagePackBridge.process('fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
