```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"astro"}');
  final result = await TreeSitterLanguagePackBridge.process('---\n---\n<p>hello</p>', config: _config);
}

```
