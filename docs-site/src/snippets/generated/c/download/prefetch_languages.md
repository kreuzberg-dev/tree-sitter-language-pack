---
id: fixture_ffi_prefetch_languages
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
    ts_pack_prefetch("[\"python\"]");
    return EXIT_SUCCESS;
}

```
