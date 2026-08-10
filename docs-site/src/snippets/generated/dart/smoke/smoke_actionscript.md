```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"actionscript"}');
  final result = await TreeSitterLanguagePackBridge.process('var x:int = 1;', config: _config);
}

```
