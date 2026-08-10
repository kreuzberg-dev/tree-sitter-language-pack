```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"sysml"}');
  final result = await TreeSitterLanguagePackBridge.process('package P {}\n', config: _config);
}

```
