```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"ninja"}');
  final result = await TreeSitterLanguagePackBridge.process('rule cc\n  command = cc \$in -o \$out', config: _config);
}

```
