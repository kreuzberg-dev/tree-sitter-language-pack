```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"csv"}');
  final result = await TreeSitterLanguagePackBridge.process('a,b,c\n1,2,3', config: _config);
}

```
