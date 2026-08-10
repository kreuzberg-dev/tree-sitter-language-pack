```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.process("<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", config)
}

```
