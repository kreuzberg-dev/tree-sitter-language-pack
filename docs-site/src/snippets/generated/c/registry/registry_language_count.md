---
id: fixture_ffi_registry_language_count
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
    uintptr_t result = ts_pack_language_count();
    return EXIT_SUCCESS;
}

```
