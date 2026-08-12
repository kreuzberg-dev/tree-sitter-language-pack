---
id: fixture_dart_data_extraction_hjson_flat
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
    final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"hjson"}');
    final result = await TreeSitterLanguagePackBridge.process('{\n  host: "localhost"\n  port: 8080\n}\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
