```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"xml"}');
  final result = await TreeSitterLanguagePackBridge.process('<server id="main"><host>localhost</host></server>', config: _config);
}

```
