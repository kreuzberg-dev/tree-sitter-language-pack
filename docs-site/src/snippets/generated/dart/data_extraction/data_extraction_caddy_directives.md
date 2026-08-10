```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"caddy"}');
  final result = await TreeSitterLanguagePackBridge.process('localhost\nroot * /var/www\nfile_server\n', config: _config);
}

```
