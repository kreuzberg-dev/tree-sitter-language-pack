---
id: fixture_dart_download_configure_custom_dir
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
    final _config = await createPackConfigFromJson(json: '{"cache_dir":"/tmp/tslp_test_cache"}');
    final result = await TreeSitterLanguagePackBridge.configure(config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
