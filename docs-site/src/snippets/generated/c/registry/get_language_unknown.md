---
id: fixture_ffi_get_language_unknown
language: c
target: ffi
level: typecheck
requires: []
side_effect: safe
---

```c title="C"
#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "ts_pack.h"

int main(void) {
    const TSLanguage *language = ts_pack_get_language("nonexistent_xyz");
    if (language != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
