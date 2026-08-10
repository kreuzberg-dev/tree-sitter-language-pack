```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", config)
}

```
