```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"cypher"}');
  final result = await TreeSitterLanguagePackBridge.process('MATCH (n) RETURN n\n', config: _config);
}

```
