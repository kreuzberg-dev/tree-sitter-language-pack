---
id: fixture_ffi_data_extraction_po_message
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
    TS_PACKProcessConfig* config_handle = ts_pack_process_config_from_json("{\"data_extraction\":true,\"language\":\"po\"}");
    TS_PACKProcessResult* result = ts_pack_process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", config_handle);
    ts_pack_process_config_free(config_handle);
    ts_pack_process_result_free(result);
    return EXIT_SUCCESS;
}

```
