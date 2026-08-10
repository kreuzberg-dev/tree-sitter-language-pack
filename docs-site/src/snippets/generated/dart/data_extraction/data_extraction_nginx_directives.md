```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"data_extraction":true,"language":"nginx"}');
  final result = await TreeSitterLanguagePackBridge.process('worker_processes 4;\nerror_log /var/log/nginx/error.log;\n', config: _config);
}

```
