---
id: fixture_dart_detect_content_python_shebang
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
    final result = await TreeSitterLanguagePackBridge.detectLanguageFromContent('#!/usr/bin/env python3\npass');
  } finally {
    RustLib.dispose();
  }
}

```
