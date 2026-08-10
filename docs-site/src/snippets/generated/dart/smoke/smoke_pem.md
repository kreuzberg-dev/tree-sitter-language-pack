```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"pem"}');
  final result = await TreeSitterLanguagePackBridge.process('-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----', config: _config);
}

```
