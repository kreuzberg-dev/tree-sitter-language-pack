package io.github.treesitter.languagepack;

import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.atomic.AtomicReference;

/**
 * Java binding for the tree-sitter-language-pack C FFI registry.
 *
 * <p>Uses the Panama Foreign Function and Memory API (JDK 22+) to call
 * into the native {@code ts_pack_ffi} library. No JNI is involved.</p>
 *
 * <p>Implements {@link AutoCloseable} so it can be used in try-with-resources blocks:</p>
 * <pre>{@code
 * try (var registry = new TsPackRegistry()) {
 *     MemorySegment lang = registry.getLanguage("java");
 * }
 * }</pre>
 */
public class TsPackRegistry implements AutoCloseable {

    private static final Linker LINKER = Linker.nativeLinker();
    private static final SymbolLookup LOOKUP;

    // Method handles for each C function
    private static final MethodHandle REGISTRY_NEW;
    private static final MethodHandle REGISTRY_FREE;
    private static final MethodHandle GET_LANGUAGE;
    private static final MethodHandle LANGUAGE_COUNT;
    private static final MethodHandle LANGUAGE_NAME_AT;
    private static final MethodHandle HAS_LANGUAGE;
    private static final MethodHandle LAST_ERROR;
    private static final MethodHandle CLEAR_ERROR;
    private static final MethodHandle FREE_STRING;

    static {
        // Load the native library: check TSPACK_LIB_PATH env var first, then system path
        String libPath = System.getenv("TSPACK_LIB_PATH");
        if (libPath != null && !libPath.isEmpty()) {
            LOOKUP = SymbolLookup.libraryLookup(Path.of(libPath), Arena.global());
        } else {
            LOOKUP = SymbolLookup.libraryLookup("ts_pack_ffi", Arena.global());
        }

        // ts_pack_registry_new() -> pointer
        REGISTRY_NEW = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_registry_new").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS)
        );

        // ts_pack_registry_free(pointer) -> void
        REGISTRY_FREE = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_registry_free").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS)
        );

        // ts_pack_get_language(pointer, pointer) -> pointer
        GET_LANGUAGE = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_get_language").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS)
        );

        // ts_pack_language_count(pointer) -> long (uintptr_t)
        LANGUAGE_COUNT = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_language_count").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS)
        );

        // ts_pack_language_name_at(pointer, long) -> pointer
        LANGUAGE_NAME_AT = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_language_name_at").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG)
        );

        // ts_pack_has_language(pointer, pointer) -> boolean
        HAS_LANGUAGE = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_has_language").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.ADDRESS)
        );

        // ts_pack_last_error() -> pointer
        LAST_ERROR = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_last_error").orElseThrow(),
                FunctionDescriptor.of(ValueLayout.ADDRESS)
        );

        // ts_pack_clear_error() -> void
        CLEAR_ERROR = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_clear_error").orElseThrow(),
                FunctionDescriptor.ofVoid()
        );

        // ts_pack_free_string(pointer) -> void
        FREE_STRING = LINKER.downcallHandle(
                LOOKUP.find("ts_pack_free_string").orElseThrow(),
                FunctionDescriptor.ofVoid(ValueLayout.ADDRESS)
        );
    }

    private final AtomicReference<MemorySegment> registryPtr;

    /**
     * Creates a new language registry by calling {@code ts_pack_registry_new()}.
     *
     * @throws RuntimeException if the native registry could not be created
     */
    public TsPackRegistry() {
        MemorySegment ptr;
        try {
            ptr = (MemorySegment) REGISTRY_NEW.invokeExact();
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke ts_pack_registry_new", t);
        }

        if (ptr.equals(MemorySegment.NULL)) {
            String error = lastError();
            throw new RuntimeException(
                    "ts_pack_registry_new returned null" + (error != null ? ": " + error : "")
            );
        }
        this.registryPtr = new AtomicReference<>(ptr);
    }

    /**
     * Frees the underlying native registry. Safe to call multiple times.
     */
    @Override
    public void close() {
        MemorySegment ptr = registryPtr.getAndSet(MemorySegment.NULL);
        if (ptr != null && !ptr.equals(MemorySegment.NULL)) {
            try {
                REGISTRY_FREE.invokeExact(ptr);
            } catch (Throwable t) {
                throw new RuntimeException("Failed to invoke ts_pack_registry_free", t);
            }
        }
    }

    /**
     * Returns the raw {@code TSLanguage*} pointer for the given language name.
     *
     * @param name the language name (e.g. "java", "python")
     * @return a {@link MemorySegment} pointing to the TSLanguage struct
     * @throws IllegalArgumentException if the language is not found or an error occurred
     */
    public MemorySegment getLanguage(String name) {
        MemorySegment ptr = ensureOpen();

        try (Arena arena = Arena.ofConfined()) {
            MemorySegment cName = arena.allocateFrom(name);
            MemorySegment result = (MemorySegment) GET_LANGUAGE.invokeExact(ptr, cName);

            if (result.equals(MemorySegment.NULL)) {
                String error = lastError();
                throw new IllegalArgumentException(
                        "Language not found: " + name + (error != null ? " (" + error + ")" : "")
                );
            }
            return result;
        } catch (IllegalArgumentException e) {
            throw e;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke ts_pack_get_language", t);
        }
    }

    /**
     * Returns the number of available languages in the registry.
     *
     * @return the language count
     */
    public int languageCount() {
        MemorySegment ptr = ensureOpen();

        try {
            long count = (long) LANGUAGE_COUNT.invokeExact(ptr);
            return Math.toIntExact(count);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke ts_pack_language_count", t);
        }
    }

    /**
     * Returns the language name at the given index.
     *
     * @param index zero-based index into the language list
     * @return the language name
     * @throws IndexOutOfBoundsException if the index is out of range
     */
    public String languageNameAt(int index) {
        MemorySegment ptr = ensureOpen();

        try {
            MemorySegment cStr = (MemorySegment) LANGUAGE_NAME_AT.invokeExact(ptr, (long) index);

            if (cStr.equals(MemorySegment.NULL)) {
                throw new IndexOutOfBoundsException("Index out of bounds: " + index);
            }

            try {
                // The returned string is a fresh allocation; read it then free it.
                String result = cStr.reinterpret(Long.MAX_VALUE).getString(0);
                return result;
            } finally {
                FREE_STRING.invokeExact(cStr);
            }
        } catch (IndexOutOfBoundsException e) {
            throw e;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke ts_pack_language_name_at", t);
        }
    }

    /**
     * Checks whether the registry contains a language with the given name.
     *
     * @param name the language name
     * @return {@code true} if the language is available
     */
    public boolean hasLanguage(String name) {
        MemorySegment ptr = ensureOpen();

        try (Arena arena = Arena.ofConfined()) {
            MemorySegment cName = arena.allocateFrom(name);
            return (boolean) HAS_LANGUAGE.invokeExact(ptr, cName);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke ts_pack_has_language", t);
        }
    }

    /**
     * Returns an unmodifiable list of all available language names.
     *
     * @return list of language names
     */
    public List<String> availableLanguages() {
        int count = languageCount();
        List<String> languages = new ArrayList<>(count);
        for (int i = 0; i < count; i++) {
            languages.add(languageNameAt(i));
        }
        return Collections.unmodifiableList(languages);
    }

    /**
     * Clears the last error on the current thread.
     */
    public static void clearError() {
        try {
            CLEAR_ERROR.invokeExact();
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke ts_pack_clear_error", t);
        }
    }

    // --- internal helpers ---

    /**
     * Reads the last error message from the FFI layer (thread-local).
     *
     * @return the error message, or {@code null} if no error
     */
    private static String lastError() {
        try {
            MemorySegment errPtr = (MemorySegment) LAST_ERROR.invokeExact();
            if (errPtr.equals(MemorySegment.NULL)) {
                return null;
            }
            // The pointer is valid until the next FFI call; do NOT free it.
            return errPtr.reinterpret(Long.MAX_VALUE).getString(0);
        } catch (Throwable t) {
            return null;
        }
    }

    private MemorySegment ensureOpen() {
        MemorySegment ptr = registryPtr.get();
        if (ptr == null || ptr.equals(MemorySegment.NULL)) {
            throw new IllegalStateException("Registry has been closed");
        }
        return ptr;
    }
}
