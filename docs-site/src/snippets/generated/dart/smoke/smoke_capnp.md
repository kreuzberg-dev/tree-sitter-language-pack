```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"capnp"}');
  final result = await TreeSitterLanguagePackBridge.process('@0xabcdef1234567890;', config: _config);
}

```
