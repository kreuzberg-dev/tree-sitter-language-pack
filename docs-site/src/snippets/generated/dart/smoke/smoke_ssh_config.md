```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"ssh_config"}');
  final result = await TreeSitterLanguagePackBridge.process('Host example\n  HostName example.com', config: _config);
}

```
