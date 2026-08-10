```kotlin title="Kotlin (Android)"
import io.xberg.tslp.android.*

fun main() = kotlinx.coroutines.runBlocking {
    val result = TreeSitterLanguagePack.detectLanguageFromContent("#!/usr/bin/env python3\npass")
}

```
