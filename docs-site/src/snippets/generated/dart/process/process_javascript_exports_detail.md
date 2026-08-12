---
id: fixture_dart_process_javascript_exports_detail
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
    final _config = await createProcessConfigFromJson(json: '{"language":"javascript"}');
    final result = await TreeSitterLanguagePackBridge.process('export function greet(name) {\n  return `Hello \${name}`;\n}\n\nexport const VERSION = \'1.0\';\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
