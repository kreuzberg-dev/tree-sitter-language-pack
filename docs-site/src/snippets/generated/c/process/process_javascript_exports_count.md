---
id: fixture_ffi_process_javascript_exports_count
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
    TS_PACKProcessConfig* config_handle = ts_pack_process_config_from_json("{\"language\":\"javascript\"}");
    TS_PACKProcessResult* result = ts_pack_process("export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", config_handle);
    ts_pack_process_config_free(config_handle);
    ts_pack_process_result_free(result);
    return EXIT_SUCCESS;
}

```
