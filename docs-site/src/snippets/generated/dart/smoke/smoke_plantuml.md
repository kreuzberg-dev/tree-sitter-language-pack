```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"plantuml"}');
  final result = await TreeSitterLanguagePackBridge.process('@startuml\n@enduml\n', config: _config);
}

```
