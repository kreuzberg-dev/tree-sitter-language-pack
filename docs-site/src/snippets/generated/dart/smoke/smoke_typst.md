```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"typst"}');
  final result = await TreeSitterLanguagePackBridge.process('#let x = 1', config: _config);
}

```
