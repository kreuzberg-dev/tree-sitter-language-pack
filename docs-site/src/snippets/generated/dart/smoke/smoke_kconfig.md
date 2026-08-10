```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"kconfig"}');
  final result = await TreeSitterLanguagePackBridge.process('config FOO\n\tbool "Enable foo"', config: _config);
}

```
