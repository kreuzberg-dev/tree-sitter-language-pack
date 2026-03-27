```javascript title="WebAssembly"
import { availableLanguages, parseString, treeRootNodeType } from "@kreuzberg/tree-sitter-language-pack-wasm";

const langs = availableLanguages();
console.log(`${langs.length} languages available`);

const tree = parseString("python", "def hello(): pass");
console.log(`Root: ${treeRootNodeType(tree)}`);
```
