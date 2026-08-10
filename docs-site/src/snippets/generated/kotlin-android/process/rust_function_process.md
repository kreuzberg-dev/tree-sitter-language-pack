```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", config)
}

```
