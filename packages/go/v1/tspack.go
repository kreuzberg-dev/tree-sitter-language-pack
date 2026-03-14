// Package tspack provides Go bindings for tree-sitter-language-pack via cgo.
//
// It wraps the C-FFI layer (ts-pack-ffi) to provide access to 165+ tree-sitter
// language grammars through a safe, idiomatic Go API.
//
// Language names are plain strings such as "python", "rust", "javascript", etc.
// Use [Registry.AvailableLanguages] to discover all supported names at runtime,
// or [Registry.HasLanguage] to check for a specific language before loading it.
//
// # Usage
//
//	reg, err := tspack.NewRegistry()
//	if err != nil {
//	    log.Fatal(err)
//	}
//	defer reg.Close()
//
//	langPtr, err := reg.GetLanguage("python")
//	if err != nil {
//	    log.Fatal(err)
//	}
//	// langPtr is an unsafe.Pointer to a TSLanguage struct that can be
//	// passed to a tree-sitter Go wrapper such as go-tree-sitter.
//
// # Concurrency
//
// The [Registry] type is safe for concurrent use from multiple goroutines.
// All exported methods acquire the appropriate lock before accessing the
// underlying C registry.
package tspack

/*
#include "ts_pack.h"
#include <stdlib.h>
#include <stdint.h>
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
// language grammars. It is safe for concurrent use from multiple goroutines.
//
// A Registry must be created via [NewRegistry] and should be closed with
// [Registry.Close] when no longer needed. If Close is not called, the
// finalizer will release the underlying C resources during garbage collection.
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

// LanguageNameAt returns the language name at the given index. Valid indices
// are in the range [0, LanguageCount()). Returns an error if the index is out
// of bounds or the registry is closed.
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

// Tree wraps an opaque TsPackTree handle representing a parsed syntax tree.
// It must be freed with Close when no longer needed.
type Tree struct {
	ptr *C.TsPackTree
}

// ParseString parses the given source code using the named language and returns
// an opaque Tree handle. The caller must call Tree.Close when done.
//
// Returns an error if the language is not found or parsing fails.
func (r *Registry) ParseString(language, source string) (*Tree, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if err := r.ensureOpen(); err != nil {
		return nil, err
	}

	cname := C.CString(language)
	defer C.free(unsafe.Pointer(cname))

	csource := C.CString(source)
	defer C.free(unsafe.Pointer(csource))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	tree := C.ts_pack_parse_string(r.ptr, cname, csource, C.uintptr_t(len(source)))

	if tree == nil {
		if err := lastError(); err != nil {
			return nil, fmt.Errorf("tspack: parse %q: %w", language, err)
		}
		return nil, fmt.Errorf("tspack: parse %q failed", language)
	}

	return &Tree{ptr: tree}, nil
}

// Close frees the underlying C tree. Safe to call multiple times.
func (t *Tree) Close() {
	if t.ptr != nil {
		C.ts_pack_tree_free(t.ptr)
		t.ptr = nil
	}
}

// RootNodeType returns the type name of the root node.
func (t *Tree) RootNodeType() (string, error) {
	if t.ptr == nil {
		return "", errors.New("tspack: tree is closed")
	}

	cstr := C.ts_pack_tree_root_node_type(t.ptr)
	if cstr == nil {
		return "", errors.New("tspack: failed to get root node type")
	}
	defer C.ts_pack_free_string(cstr)

	return C.GoString(cstr), nil
}

// RootChildCount returns the number of named children of the root node.
func (t *Tree) RootChildCount() (int, error) {
	if t.ptr == nil {
		return 0, errors.New("tspack: tree is closed")
	}

	return int(C.ts_pack_tree_root_child_count(t.ptr)), nil
}

// ContainsNodeType checks whether any node in the tree has the given type name.
func (t *Tree) ContainsNodeType(nodeType string) (bool, error) {
	if t.ptr == nil {
		return false, errors.New("tspack: tree is closed")
	}

	ctype := C.CString(nodeType)
	defer C.free(unsafe.Pointer(ctype))

	return bool(C.ts_pack_tree_contains_node_type(t.ptr, ctype)), nil
}

// HasErrorNodes checks whether the tree contains any ERROR or MISSING nodes.
func (t *Tree) HasErrorNodes() (bool, error) {
	if t.ptr == nil {
		return false, errors.New("tspack: tree is closed")
	}

	return bool(C.ts_pack_tree_has_error_nodes(t.ptr)), nil
}

// Process extracts file intelligence from the given source code and returns
// the result as a JSON string.
//
// Returns an error if the language is not found or processing fails.
func (r *Registry) Process(source, language string) (string, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if err := r.ensureOpen(); err != nil {
		return "", err
	}

	csource := C.CString(source)
	defer C.free(unsafe.Pointer(csource))

	clang := C.CString(language)
	defer C.free(unsafe.Pointer(clang))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	result := C.ts_pack_process(r.ptr, csource, C.uintptr_t(len(source)), clang)

	if result == nil {
		if err := lastError(); err != nil {
			return "", fmt.Errorf("tspack: process %q: %w", language, err)
		}
		return "", fmt.Errorf("tspack: process %q failed", language)
	}
	defer C.ts_pack_free_string(result)

	return C.GoString(result), nil
}

// ProcessAndChunk extracts file intelligence and performs AST-aware chunking,
// returning the result as a JSON string containing both intelligence and chunks.
//
// Returns an error if the language is not found or processing fails.
func (r *Registry) ProcessAndChunk(source, language string, maxChunkSize int) (string, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	if err := r.ensureOpen(); err != nil {
		return "", err
	}

	csource := C.CString(source)
	defer C.free(unsafe.Pointer(csource))

	clang := C.CString(language)
	defer C.free(unsafe.Pointer(clang))

	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	result := C.ts_pack_process_and_chunk(r.ptr, csource, C.uintptr_t(len(source)), clang, C.uintptr_t(maxChunkSize))

	if result == nil {
		if err := lastError(); err != nil {
			return "", fmt.Errorf("tspack: process_and_chunk %q: %w", language, err)
		}
		return "", fmt.Errorf("tspack: process_and_chunk %q failed", language)
	}
	defer C.ts_pack_free_string(result)

	return C.GoString(result), nil
}
