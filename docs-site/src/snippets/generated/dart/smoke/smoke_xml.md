```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"xml"}');
  final result = await TreeSitterLanguagePackBridge.process('<?xml version="1.0"?>\n<root>hello</root>', config: _config);
}

```
