```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"qmljs"}');
  final result = await TreeSitterLanguagePackBridge.process('import QtQuick 2.0\nItem {}', config: _config);
}

```
