---
id: fixture_dart_error_empty_language_name
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'dart:io';
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
import 'package:tree_sitter_language_pack/src/tree_sitter_language_pack_bridge_generated/frb_generated.dart' show RustLib;
Future<void> main() async {
  await RustLib.init();
  try {
    try {
      final _config = await createProcessConfigFromJson(json: '{"language":""}');
      final result = await TreeSitterLanguagePackBridge.process('hello', config: _config);
    } catch (error) {
      stderr.writeln('Call failed as expected: $error');
      return;
    }
    throw StateError('expected call to fail');
  } finally {
    RustLib.dispose();
  }
}

```
