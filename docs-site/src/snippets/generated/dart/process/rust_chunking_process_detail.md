```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"chunk_max_size":30,"language":"rust"}');
  final result = await TreeSitterLanguagePackBridge.process('fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n', config: _config);
}

```
