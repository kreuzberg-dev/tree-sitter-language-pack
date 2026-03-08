//go:build tspack_dev

package tspack

/*
#cgo CFLAGS: -I${SRCDIR}/../ts-pack-ffi/include
#cgo darwin LDFLAGS: ${SRCDIR}/../ts-pack-ffi/target/release/libts_pack_ffi.a -lc++
#cgo linux LDFLAGS: -L${SRCDIR}/../ts-pack-ffi/target/release -Wl,-Bstatic -lts_pack_ffi -Wl,-Bdynamic -lpthread -ldl -lm -lstdc++
#cgo windows LDFLAGS: -L${SRCDIR}/../ts-pack-ffi/target/release -lts_pack_ffi -lws2_32 -lbcrypt -lntdll -static-libgcc -static-libstdc++
#include "ts_pack.h"
#include <stdlib.h>
#include <stdint.h>
*/
import "C"
