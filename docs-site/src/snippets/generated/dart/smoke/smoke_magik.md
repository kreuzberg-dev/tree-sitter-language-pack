```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"magik"}');
  final result = await TreeSitterLanguagePackBridge.process('_method object.hello\n_endmethod', config: _config);
}

```
