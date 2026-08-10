```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"solidity"}');
  final result = await TreeSitterLanguagePackBridge.process('pragma solidity ^0.8.0;\ncontract Main {}', config: _config);
}

```
