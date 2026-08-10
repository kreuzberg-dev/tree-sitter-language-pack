```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"postscript"}');
  final result = await TreeSitterLanguagePackBridge.process('/hello { (Hello) show } def', config: _config);
}

```
