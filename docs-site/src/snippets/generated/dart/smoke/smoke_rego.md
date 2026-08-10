```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"rego"}');
  final result = await TreeSitterLanguagePackBridge.process('package main\ndefault allow = false', config: _config);
}

```
