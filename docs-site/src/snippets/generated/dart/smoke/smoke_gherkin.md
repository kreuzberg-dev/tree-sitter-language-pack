---
id: fixture_dart_smoke_gherkin
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
    final _config = await createProcessConfigFromJson(json: '{"language":"gherkin"}');
    final result = await TreeSitterLanguagePackBridge.process('Feature: Calculator\n  Scenario: Add numbers\n    Given I have entered 1\n    When I add 2\n    Then the result should be 3\n', config: _config);
  } finally {
    RustLib.dispose();
  }
}

```
