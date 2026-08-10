```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"latex"}');
  final result = await TreeSitterLanguagePackBridge.process('\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}', config: _config);
}

```
