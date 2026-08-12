---
id: fixture_dart_data_extraction_editorconfig_section
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
    final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"editorconfig"}');
    final result = await TreeSitterLanguagePackBridge.process('[*.rs]\nindent_style = space\nindent_size = 4\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
