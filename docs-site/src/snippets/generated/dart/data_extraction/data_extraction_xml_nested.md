```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"xml"}');
  final result = await TreeSitterLanguagePackBridge.process('<config><host>localhost</host><port>8080</port></config>', config: _config);
}

```
