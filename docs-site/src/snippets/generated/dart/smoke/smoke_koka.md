```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"koka"}');
  final result = await TreeSitterLanguagePackBridge.process('fun main()\n  1\n', config: _config);
}

```
