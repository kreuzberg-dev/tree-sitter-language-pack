---
id: fixture_ffi_detect_ext_ruby
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
    char* result = ts_pack_detect_language_from_extension("rb");
    if (result != NULL) { ts_pack_free_string(result); }
    return EXIT_SUCCESS;
}

```
