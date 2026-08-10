```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"scfg"}');
  final result = await TreeSitterLanguagePackBridge.process('key value\n', config: _config);
}

```
