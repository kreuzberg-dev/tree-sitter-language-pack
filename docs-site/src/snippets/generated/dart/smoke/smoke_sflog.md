```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"sflog"}');
  final result = await TreeSitterLanguagePackBridge.process('37.0 APEX_CODE,DEBUG\n16:06:58.18 (1)|EXECUTION_STARTED\n', config: _config);
}

```
