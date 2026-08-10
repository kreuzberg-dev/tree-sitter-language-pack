```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"beancount"}');
  final result = await TreeSitterLanguagePackBridge.process('2024-01-01 open Assets:Bank USD', config: _config);
}

```
