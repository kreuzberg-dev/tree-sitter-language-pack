```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"tsv"}');
  final result = await TreeSitterLanguagePackBridge.process('a\tb\tc\n1\t2\t3', config: _config);
}

```
