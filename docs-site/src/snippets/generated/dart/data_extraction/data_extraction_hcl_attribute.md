```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"hcl"}');
  final result = await TreeSitterLanguagePackBridge.process('region = "us-east-1"\ncount  = 3\n', config: _config);
}

```
