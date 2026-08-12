---
id: fixture_ffi_registry_has_language_true
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
    int32_t result = ts_pack_has_language("python");
    return EXIT_SUCCESS;
}

```
