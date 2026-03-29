```typescript title="Node.js"
import { init, download, downloadedLanguages, manifestLanguages } from "@kreuzberg/tree-sitter-language-pack";

// Pre-download specific languages
download(["python", "javascript", "rust"]);

// Or initialize with config
init({ languages: ["python", "go"], cacheDir: "/tmp/parsers" });

// Check what's cached
console.log(downloadedLanguages());
console.log(manifestLanguages().slice(0, 5));
```
