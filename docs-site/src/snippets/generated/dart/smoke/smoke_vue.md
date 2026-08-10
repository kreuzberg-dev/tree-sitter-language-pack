```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"vue"}');
  final result = await TreeSitterLanguagePackBridge.process('<template><div>hello</div></template>', config: _config);
}

```
