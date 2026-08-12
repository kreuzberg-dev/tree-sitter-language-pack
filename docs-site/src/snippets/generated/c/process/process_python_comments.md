---
id: fixture_ffi_process_python_comments
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
    TS_PACKProcessConfig* config_handle = ts_pack_process_config_from_json("{\"comments\":true,\"language\":\"python\"}");
    TS_PACKProcessResult* result = ts_pack_process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", config_handle);
    ts_pack_process_config_free(config_handle);
    ts_pack_process_result_free(result);
    return EXIT_SUCCESS;
}

```
