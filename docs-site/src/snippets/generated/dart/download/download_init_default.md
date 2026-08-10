```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createPackConfigFromJson(json: '{}');
  final result = await TreeSitterLanguagePackBridge.init(config: _config);
}

```
