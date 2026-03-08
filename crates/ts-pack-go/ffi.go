//go:build !windows && !tspack_dev

package tspack

/*
#cgo CFLAGS: -I${SRCDIR}/include
#cgo darwin LDFLAGS: ${SRCDIR}/lib/libts_pack_ffi.a -lc++
#cgo linux LDFLAGS: -L${SRCDIR}/lib -Wl,-Bstatic -lts_pack_ffi -Wl,-Bdynamic -lpthread -ldl -lm -lstdc++
#include "ts_pack.h"
#include <stdlib.h>
#include <stdint.h>
*/
import "C"
