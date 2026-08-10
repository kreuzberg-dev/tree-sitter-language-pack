```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"gdscript"}');
  final result = await TreeSitterLanguagePackBridge.process('extends Node\nfunc _ready():\n\tpass', config: _config);
}

```
