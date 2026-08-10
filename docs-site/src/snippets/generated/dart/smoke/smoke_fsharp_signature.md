```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"fsharp_signature"}');
  final result = await TreeSitterLanguagePackBridge.process('val x: int', config: _config);
}

```
