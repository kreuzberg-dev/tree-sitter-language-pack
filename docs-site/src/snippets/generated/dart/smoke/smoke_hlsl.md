```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
Future<void> main() async {
  final _config = await createProcessConfigFromJson(json: '{"language":"hlsl"}');
  final result = await TreeSitterLanguagePackBridge.process('float4 main() : SV_Target { return 0; }', config: _config);
}

```
