```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"c"}');
  final result = await TreeSitterLanguagePackBridge.process('#include <stdio.h>\n\nint main() {\n    printf("hello");\n    return 0;\n}\n', config: _config);
}

```
