---
id: fixture_ffi_get_parser_python
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
    TS_PACKParser* parser = ts_pack_get_parser("python");
    ts_pack_parser_free(parser);
    return EXIT_SUCCESS;
}

```
