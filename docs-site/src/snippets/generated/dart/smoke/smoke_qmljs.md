---
id: fixture_dart_smoke_qmljs
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
    final _config = await createProcessConfigFromJson(json: '{"language":"qmljs"}');
    final result = await TreeSitterLanguagePackBridge.process('import QtQuick 2.0\nItem {}', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
