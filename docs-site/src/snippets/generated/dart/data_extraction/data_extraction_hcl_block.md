---
id: fixture_dart_data_extraction_hcl_block
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
    final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"hcl"}');
    final result = await TreeSitterLanguagePackBridge.process('resource "aws_instance" "web" {\n  ami = "ami-123"\n  instance_type = "t2.micro"\n}\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
