---
id: fixture_ffi_error_handling_get_language_empty_string
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
    const TSLanguage *language = ts_pack_get_language("");
    if (language != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
