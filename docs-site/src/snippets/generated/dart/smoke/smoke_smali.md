```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"smali"}');
  final result = await TreeSitterLanguagePackBridge.process('.class public LMain;\n.super Ljava/lang/Object;', config: _config);
}

```
