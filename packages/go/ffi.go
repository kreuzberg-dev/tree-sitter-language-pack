//go:build !windows && !tspack_dev

package tspack

/*
#cgo CFLAGS: -I${SRCDIR}/include
#cgo darwin LDFLAGS: ${SRCDIR}/lib/libts_pack_ffi.a -lc++
#cgo linux LDFLAGS: -L${SRCDIR}/lib -Wl,-Bstatic -lts_pack_ffi -Wl,-Bdynamic -lpthread -ldl -lm -lstdc++
#include "ts_pack.h"
#include <stdlib.h>
#include <stdint.h>

// Forward declarations for download/configure API
int32_t ts_pack_init(const char *config_json);
int32_t ts_pack_configure(const char *config_json);
int32_t ts_pack_download(const char *const *names, uintptr_t count);
int32_t ts_pack_download_all(void);
const char *const *ts_pack_manifest_languages(uintptr_t *out_count);
const char *const *ts_pack_downloaded_languages(uintptr_t *out_count);
int32_t ts_pack_clean_cache(void);
const char *ts_pack_cache_dir(void);
void ts_pack_free_string_array(const char **arr);
*/
import "C"
