```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"python"}');
  final result = await TreeSitterLanguagePackBridge.process('# module docstring\nimport os\n\ndef hello():\n    # greeting\n    print(\'hello\')\n\ndef world():\n    print(\'world\')\n', config: _config);
}

```
