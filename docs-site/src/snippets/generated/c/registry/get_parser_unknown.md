---
id: fixture_ffi_get_parser_unknown
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
    TS_PACKParser* parser = ts_pack_get_parser("nonexistent_xyz");
    if (parser != NULL) { return EXIT_FAILURE; }
    return EXIT_SUCCESS;
}

```
