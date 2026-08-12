---
id: fixture_ffi_java_package_declaration_process
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
    TS_PACKProcessConfig* config_handle = ts_pack_process_config_from_json("{\"language\":\"java\"}");
    TS_PACKProcessResult* result = ts_pack_process("package com.example.widget;\n\npublic class Widget {\n    public String name() { return \"w\"; }\n}\n", config_handle);
    ts_pack_process_config_free(config_handle);
    ts_pack_process_result_free(result);
    return EXIT_SUCCESS;
}

```
