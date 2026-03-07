// Package tspack provides Go bindings for tree-sitter-language-pack via cgo.
//
// It wraps the C-FFI layer (ts-pack-ffi) to provide access to 165+ tree-sitter
// language grammars through a safe, idiomatic Go API.
//
// The Registry type is safe for concurrent use from multiple goroutines.
package tspack

/*
#cgo CFLAGS: -I${SRCDIR}/../ts-pack-ffi/include
#cgo LDFLAGS: -L${SRCDIR}/../ts-pack-ffi/target/release -lts_pack_ffi

#include "ts_pack.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"fmt"
	"runtime"
	"sync"
	"unsafe"
)

// Registry wraps a TsPackRegistry handle and provides access to tree-sitter
// language grammars. It is safe for concurrent use.
type Registry struct {
	mu  sync.RWMutex
	ptr *C.TsPackRegistry
}

// lastError retrieves the last error message from the FFI layer.
// Returns nil if no error is set.
//
// IMPORTANT: The caller must hold the OS thread locked (runtime.LockOSThread)
// because ts_pack_last_error uses thread-local storage. This function must be
// called on the same OS thread as the FFI call that produced the error.
func lastError() error {
	cerr := C.ts_pack_last_error()
	if cerr == nil {
		return nil
	}
	msg := C.GoString(cerr)
	return errors.New(msg)
}

// NewRegistry creates a new language registry containing all available
// tree-sitter grammars. The registry is automatically freed when garbage
// collected, but callers may also call Close for deterministic cleanup.
//
// Returns an error if the underlying FFI call fails.
func NewRegistry() (*Registry, error) {
	// Lock OS thread so the FFI call and subsequent error check use the same
	// thread-local storage.
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	ptr := C.ts_pack_registry_new()
	if ptr == nil {
		if err := lastError(); err != nil {
			return nil, fmt.Errorf("tspack: failed to create registry: %w", err)
		}
		return nil, errors.New("tspack: failed to create registry: unknown error")
	}

	r := &Registry{ptr: ptr}
	runtime.SetFinalizer(r, (*Registry).free)
	return r, nil
}

// free releases the underlying C registry. Called by the finalizer.
func (r *Registry) free() {
	r.mu.Lock()
	defer r.mu.Unlock()

	if r.ptr != nil {
		C.ts_pack_registry_free(r.ptr)
		r.ptr = nil
	}
}

// Close explicitly frees the underlying C registry. After Close is called,
// all other methods will return errors or zero values.
//
// It is safe to call Close multiple times.
func (r *Registry) Close() {
	r.free()
	runtime.SetFinalizer(r, nil)
}

// ensureOpen returns an error if the registry has been closed.
func (r *Registry) ensureOpen() error {
	if r.ptr == nil {
		return errors.New("tspack: registry is closed")
	}
	return nil
}

// GetLanguage returns a pointer to the TSLanguage for the given language name.
//
// The returned unsafe.Pointer can be cast to the appropriate type by consumers
// (e.g., go-tree-sitter's Language type). The pointer remains valid for the
// lifetime of the Registry.
//
// Returns an error if the language is not found or the registry is closed.
func (r *Registry) GetLanguage(name string) (unsafe.Pointer, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if err := r.ensureOpen(); err != nil {
		return nil, err
	}

	cname := C.CString(name)
	defer C.free(unsafe.Pointer(cname))

	// Lock OS thread so the FFI call and error check share thread-local storage.
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	lang := C.ts_pack_get_language(r.ptr, cname)

	if lang == nil {
		if err := lastError(); err != nil {
			return nil, fmt.Errorf("tspack: language %q: %w", name, err)
		}
		return nil, fmt.Errorf("tspack: language %q not found", name)
	}

	return unsafe.Pointer(lang), nil
}

// LanguageCount returns the number of available languages in the registry.
// Returns 0 if the registry is closed.
func (r *Registry) LanguageCount() int {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if r.ptr == nil {
		return 0
	}

	return int(C.ts_pack_language_count(r.ptr))
}

// LanguageNameAt returns the language name at the given index.
// Returns an error if the index is out of bounds or the registry is closed.
func (r *Registry) LanguageNameAt(index int) (string, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if err := r.ensureOpen(); err != nil {
		return "", err
	}

	cname := C.ts_pack_language_name_at(r.ptr, C.uintptr_t(index))

	if cname == nil {
		return "", fmt.Errorf("tspack: index %d out of bounds", index)
	}

	name := C.GoString(cname)
	C.ts_pack_free_string((*C.char)(unsafe.Pointer(cname)))

	return name, nil
}

// HasLanguage reports whether the registry contains a grammar for the named
// language. Returns false if the registry is closed.
func (r *Registry) HasLanguage(name string) bool {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if r.ptr == nil {
		return false
	}

	cname := C.CString(name)
	defer C.free(unsafe.Pointer(cname))

	return bool(C.ts_pack_has_language(r.ptr, cname))
}

// AvailableLanguages returns a slice of all language names in the registry.
// Returns nil if the registry is closed.
func (r *Registry) AvailableLanguages() []string {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if r.ptr == nil {
		return nil
	}

	count := int(C.ts_pack_language_count(r.ptr))
	if count == 0 {
		return nil
	}

	languages := make([]string, 0, count)
	for i := 0; i < count; i++ {
		cname := C.ts_pack_language_name_at(r.ptr, C.uintptr_t(i))
		if cname == nil {
			continue
		}
		languages = append(languages, C.GoString(cname))
		C.ts_pack_free_string((*C.char)(unsafe.Pointer(cname)))
	}

	return languages
}
