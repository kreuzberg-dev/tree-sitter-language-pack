---
id: fixture_ffi_download_multiple_languages
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
    uintptr_t result = ts_pack_download("[\"python\",\"rust\"]");
    return EXIT_SUCCESS;
}

```
