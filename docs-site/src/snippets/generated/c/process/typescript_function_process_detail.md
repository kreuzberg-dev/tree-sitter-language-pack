---
id: fixture_ffi_typescript_function_process_detail
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
    TS_PACKProcessConfig* config_handle = ts_pack_process_config_from_json("{\"language\":\"typescript\"}");
    TS_PACKProcessResult* result = ts_pack_process("import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, ${name}!`;\n}\n", config_handle);
    ts_pack_process_config_free(config_handle);
    ts_pack_process_result_free(result);
    return EXIT_SUCCESS;
}

```
