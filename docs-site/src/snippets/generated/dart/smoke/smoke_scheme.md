```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"scheme"}');
  final result = await TreeSitterLanguagePackBridge.process('(define x 1)', config: _config);
}

```
