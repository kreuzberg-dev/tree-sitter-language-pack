```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final result = await TreeSitterLanguagePackBridge.detectLanguageFromContent('#!/usr/bin/env python3\npass');
}

```
