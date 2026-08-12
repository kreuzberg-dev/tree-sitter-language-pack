---
id: fixture_dart_smoke_ssh_config
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
    final _config = await createProcessConfigFromJson(json: '{"language":"ssh_config"}');
    final result = await TreeSitterLanguagePackBridge.process('Host example\n  HostName example.com', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
