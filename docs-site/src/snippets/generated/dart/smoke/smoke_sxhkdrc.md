```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"sxhkdrc"}');
  final result = await TreeSitterLanguagePackBridge.process('super + a\n\techo hi\n', config: _config);
}

```
