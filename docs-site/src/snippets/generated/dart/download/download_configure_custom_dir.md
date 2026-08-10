```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createPackConfigFromJson(json: '{"cache_dir":"/tmp/tslp_test_cache"}');
  final result = await TreeSitterLanguagePackBridge.configure(config: _config);
}

```
