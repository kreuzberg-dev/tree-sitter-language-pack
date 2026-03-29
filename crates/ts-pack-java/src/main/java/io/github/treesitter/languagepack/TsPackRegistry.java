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
 * <p>Provides access to 165+ tree-sitter language grammars. Uses the Panama Foreign Function and
 * Memory API (JDK 22+) to call into the native {@code ts_pack_ffi} library. No JNI is involved.
 *
 * <p>Language names are plain strings such as {@code "java"}, {@code "python"}, {@code "rust"},
 * etc. Use {@link #availableLanguages()} to discover all supported names at runtime, or {@link
 * #hasLanguage(String)} to check for a specific language before loading it.
 *
 * <p>Implements {@link AutoCloseable} so it can be used in try-with-resources blocks:
 *
 * <pre>{@code
 * try (var registry = new TsPackRegistry()) {
 *     MemorySegment lang = registry.getLanguage("java");
 *     // pass lang to a tree-sitter Java wrapper
 * }
 * }</pre>
 *
 * <p>This class is <strong>not</strong> thread-safe. If concurrent access is required, callers must
 * provide their own synchronization.
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
  private static final MethodHandle PARSE_STRING;
  private static final MethodHandle PROCESS;
  private static final MethodHandle INIT;
  private static final MethodHandle CONFIGURE;
  private static final MethodHandle DOWNLOAD;
  private static final MethodHandle DOWNLOAD_ALL;
  private static final MethodHandle MANIFEST_LANGUAGES;
  private static final MethodHandle DOWNLOADED_LANGUAGES;
  private static final MethodHandle CLEAN_CACHE;
  private static final MethodHandle CACHE_DIR;
  private static final MethodHandle FREE_STRING_ARRAY;
  private static final MethodHandle DETECT_LANGUAGE;
  private static final MethodHandle DETECT_LANGUAGE_FROM_CONTENT;
  private static final MethodHandle DETECT_LANGUAGE_FROM_EXTENSION;
  private static final MethodHandle DETECT_LANGUAGE_FROM_PATH;
  private static final MethodHandle EXTENSION_AMBIGUITY;
  private static final MethodHandle GET_HIGHLIGHTS_QUERY;
  private static final MethodHandle GET_INJECTIONS_QUERY;
  private static final MethodHandle GET_LOCALS_QUERY;
  private static final MethodHandle EXTRACT;
  private static final MethodHandle VALIDATE_EXTRACTION;

  static {
    // Load the native library: check TSPACK_LIB_PATH env var first, then system path
    String libPath = System.getenv("TSPACK_LIB_PATH");
    if (libPath != null && !libPath.isEmpty()) {
      LOOKUP = SymbolLookup.libraryLookup(Path.of(libPath), Arena.global());
    } else {
      LOOKUP = SymbolLookup.libraryLookup("ts_pack_ffi", Arena.global());
    }

    // ts_pack_registry_new() -> pointer
    REGISTRY_NEW =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_registry_new").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS));

    // ts_pack_registry_free(pointer) -> void
    REGISTRY_FREE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_registry_free").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

    // ts_pack_get_language(pointer, pointer) -> pointer
    GET_LANGUAGE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_get_language").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_language_count(pointer) -> long (uintptr_t)
    LANGUAGE_COUNT =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_language_count").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_LONG, ValueLayout.ADDRESS));

    // ts_pack_language_name_at(pointer, long) -> pointer
    LANGUAGE_NAME_AT =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_language_name_at").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG));

    // ts_pack_has_language(pointer, pointer) -> boolean
    HAS_LANGUAGE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_has_language").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.JAVA_BOOLEAN, ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_last_error() -> pointer
    LAST_ERROR =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_last_error").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS));

    // ts_pack_clear_error() -> void
    CLEAR_ERROR =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_clear_error").orElseThrow(), FunctionDescriptor.ofVoid());

    // ts_pack_free_string(pointer) -> void
    FREE_STRING =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_free_string").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

    // ts_pack_parse_string(pointer, pointer, pointer, long) -> pointer
    PARSE_STRING =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_parse_string").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.ADDRESS,
                ValueLayout.ADDRESS,
                ValueLayout.ADDRESS,
                ValueLayout.ADDRESS,
                ValueLayout.JAVA_LONG));

    // ts_pack_process(pointer, pointer, long, pointer) -> pointer
    PROCESS =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_process").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.ADDRESS,
                ValueLayout.ADDRESS,
                ValueLayout.ADDRESS,
                ValueLayout.JAVA_LONG,
                ValueLayout.ADDRESS));

    // ts_pack_init(pointer) -> int32
    INIT =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_init").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS));

    // ts_pack_configure(pointer) -> int32
    CONFIGURE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_configure").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS));

    // ts_pack_download(pointer, long) -> int32
    DOWNLOAD =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_download").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.JAVA_LONG));

    // ts_pack_download_all() -> int32
    DOWNLOAD_ALL =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_download_all").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT));

    // ts_pack_manifest_languages(pointer) -> pointer
    MANIFEST_LANGUAGES =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_manifest_languages").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_downloaded_languages(pointer) -> pointer
    DOWNLOADED_LANGUAGES =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_downloaded_languages").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_clean_cache() -> int32
    CLEAN_CACHE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_clean_cache").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT));

    // ts_pack_cache_dir() -> pointer
    CACHE_DIR =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_cache_dir").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS));

    // ts_pack_free_string_array(pointer) -> void
    FREE_STRING_ARRAY =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_free_string_array").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));

    // ts_pack_detect_language(pointer) -> pointer
    DETECT_LANGUAGE =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_detect_language").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_detect_language_from_content(pointer) -> pointer
    DETECT_LANGUAGE_FROM_CONTENT =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_detect_language_from_content").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_detect_language_from_extension(pointer) -> pointer
    DETECT_LANGUAGE_FROM_EXTENSION =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_detect_language_from_extension").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_detect_language_from_path(pointer) -> pointer
    DETECT_LANGUAGE_FROM_PATH =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_detect_language_from_path").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_extension_ambiguity(pointer) -> pointer
    EXTENSION_AMBIGUITY =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_extension_ambiguity").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_get_highlights_query(pointer) -> pointer
    GET_HIGHLIGHTS_QUERY =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_get_highlights_query").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_get_injections_query(pointer) -> pointer
    GET_INJECTIONS_QUERY =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_get_injections_query").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_get_locals_query(pointer) -> pointer
    GET_LOCALS_QUERY =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_get_locals_query").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    // ts_pack_extract(pointer, long, pointer) -> pointer
    EXTRACT =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_extract").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.ADDRESS,
                ValueLayout.ADDRESS,
                ValueLayout.JAVA_LONG,
                ValueLayout.ADDRESS));

    // ts_pack_validate_extraction(pointer) -> pointer
    VALIDATE_EXTRACTION =
        LINKER.downcallHandle(
            LOOKUP.find("ts_pack_validate_extraction").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));
  }

  private static final System.Logger LOGGER = System.getLogger(TsPackRegistry.class.getName());

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
          "ts_pack_registry_new returned null" + (error != null ? ": " + error : ""));
    }
    this.registryPtr = new AtomicReference<>(ptr);
  }

  /**
   * Frees the underlying native registry. Safe to call multiple times.
   *
   * <p>After this method returns, all other instance methods will throw {@link
   * IllegalStateException}.
   *
   * @throws RuntimeException if the native free call fails
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
   * <p>The returned {@link MemorySegment} remains valid for the lifetime of this registry. It can
   * be passed to tree-sitter Java wrappers that accept a language pointer.
   *
   * @param name the language name (e.g. {@code "java"}, {@code "python"})
   * @return a {@link MemorySegment} pointing to the native {@code TSLanguage} struct
   * @throws LanguageNotFoundException if the language is not found
   * @throws IllegalStateException if the registry has been closed
   * @throws RuntimeException if the native call fails
   */
  public MemorySegment getLanguage(String name) {
    MemorySegment ptr = ensureOpen();

    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cName = arena.allocateFrom(name);
      MemorySegment result = (MemorySegment) GET_LANGUAGE.invokeExact(ptr, cName);

      if (result.equals(MemorySegment.NULL)) {
        String error = lastError();
        throw error != null
            ? new LanguageNotFoundException(name, error)
            : new LanguageNotFoundException(name);
      }
      return result;
    } catch (LanguageNotFoundException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_get_language", t);
    }
  }

  /**
   * Returns the number of available languages in the registry.
   *
   * @return the language count (always non-negative)
   * @throws IllegalStateException if the registry has been closed
   * @throws RuntimeException if the native call fails
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
   * @param index zero-based index into the language list, must be in the range {@code [0,
   *     languageCount())}
   * @return the language name (never {@code null} or empty)
   * @throws IndexOutOfBoundsException if {@code index < 0} or {@code index >= languageCount()}
   * @throws IllegalStateException if the registry has been closed
   * @throws RuntimeException if the native call fails
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
   * @param name the language name (e.g. {@code "java"}, {@code "python"})
   * @return {@code true} if the language is available, {@code false} otherwise
   * @throws IllegalStateException if the registry has been closed
   * @throws RuntimeException if the native call fails
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
   * @return an unmodifiable {@link List} of language names (never {@code null})
   * @throws IllegalStateException if the registry has been closed
   * @throws RuntimeException if the native call fails
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
   * Parses source code using the named language and returns a tree handle.
   *
   * <p>The returned {@link TsPackTree} must be closed when no longer needed.
   *
   * @param language the language name (e.g. {@code "python"}, {@code "java"})
   * @param source the source code to parse
   * @return a {@link TsPackTree} handle for inspecting the parsed syntax tree
   * @throws LanguageNotFoundException if the language is not found
   * @throws IllegalStateException if the registry has been closed
   * @throws RuntimeException if parsing fails
   */
  public TsPackTree parseString(String language, String source) {
    MemorySegment ptr = ensureOpen();

    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cName = arena.allocateFrom(language);
      MemorySegment cSource = arena.allocateFrom(source);
      MemorySegment result =
          (MemorySegment)
              PARSE_STRING.invokeExact(
                  ptr,
                  cName,
                  cSource,
                  (long) source.getBytes(java.nio.charset.StandardCharsets.UTF_8).length);

      if (result.equals(MemorySegment.NULL)) {
        String error = lastError();
        if (error != null && error.contains("not found")) {
          throw new LanguageNotFoundException(language, error);
        }
        throw new RuntimeException(
            "ts_pack_parse_string returned null" + (error != null ? ": " + error : ""));
      }
      return new TsPackTree(result);
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_parse_string", t);
    }
  }

  /**
   * Processes source code and extracts file intelligence as a JSON string.
   *
   * <p>The {@code configJson} parameter is a JSON string containing at least a {@code "language"}
   * field. Optional fields include {@code "structure"}, {@code "imports"}, {@code "exports"},
   * {@code "comments"}, {@code "docstrings"}, {@code "symbols"}, {@code "diagnostics"} (booleans,
   * default true) and {@code "chunk_max_size"} (integer, optional).
   *
   * @param source the source code to process
   * @param configJson a JSON string specifying the processing configuration
   * @return a JSON string containing the processing result
   * @throws IllegalStateException if the registry has been closed
   * @throws RuntimeException if processing fails
   */
  public String process(String source, String configJson) {
    MemorySegment ptr = ensureOpen();

    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cSource = arena.allocateFrom(source);
      MemorySegment cConfig = arena.allocateFrom(configJson);
      MemorySegment result =
          (MemorySegment)
              PROCESS.invokeExact(
                  ptr,
                  cSource,
                  (long) source.getBytes(java.nio.charset.StandardCharsets.UTF_8).length,
                  cConfig);

      if (result.equals(MemorySegment.NULL)) {
        String error = lastError();
        throw new RuntimeException(
            "ts_pack_process returned null" + (error != null ? ": " + error : ""));
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_process", t);
    }
  }

  /**
   * Detects the language name for the given file path based on its extension.
   *
   * @param path the file path (e.g. {@code "main.py"}, {@code "/src/App.java"})
   * @return the detected language name, or {@code null} if not recognized
   * @throws RuntimeException if the native call fails
   */
  public static String detectLanguage(String path) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cPath = arena.allocateFrom(path);
      MemorySegment result = (MemorySegment) DETECT_LANGUAGE.invokeExact(cPath);

      if (result.equals(MemorySegment.NULL)) {
        return null;
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_detect_language", t);
    }
  }

  /**
   * Detects the language name from file content using shebang-based detection.
   *
   * @param content the file content to analyze (e.g. {@code "#!/usr/bin/env python3\nprint('hi')"})
   * @return the detected language name, or {@code null} if not recognized
   * @throws RuntimeException if the native call fails
   */
  public static String detectLanguageFromContent(String content) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cContent = arena.allocateFrom(content);
      MemorySegment result = (MemorySegment) DETECT_LANGUAGE_FROM_CONTENT.invokeExact(cContent);

      if (result.equals(MemorySegment.NULL)) {
        return null;
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_detect_language_from_content", t);
    }
  }

  /**
   * Detects the language name from a bare file extension (without leading dot).
   *
   * @param ext the file extension (e.g. {@code "py"}, {@code "java"}, {@code "rs"})
   * @return the detected language name, or {@code null} if not recognized
   * @throws RuntimeException if the native call fails
   */
  public static String detectLanguageFromExtension(String ext) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cExt = arena.allocateFrom(ext);
      MemorySegment result = (MemorySegment) DETECT_LANGUAGE_FROM_EXTENSION.invokeExact(cExt);

      if (result.equals(MemorySegment.NULL)) {
        return null;
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_detect_language_from_extension", t);
    }
  }

  /**
   * Detects the language name from a file path based on its extension.
   *
   * <p>This is an explicit alias of {@link #detectLanguage(String)} for API consistency.
   *
   * @param path the file path (e.g. {@code "main.py"}, {@code "/src/App.java"})
   * @return the detected language name, or {@code null} if not recognized
   * @throws RuntimeException if the native call fails
   */
  public static String detectLanguageFromPath(String path) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cPath = arena.allocateFrom(path);
      MemorySegment result = (MemorySegment) DETECT_LANGUAGE_FROM_PATH.invokeExact(cPath);

      if (result.equals(MemorySegment.NULL)) {
        return null;
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_detect_language_from_path", t);
    }
  }

  /**
   * Returns ambiguity information for a file extension as a JSON string.
   *
   * <p>If the extension maps to multiple languages, the returned JSON describes the ambiguity.
   * Returns {@code null} if the extension is unambiguous or not recognized.
   *
   * @param ext the file extension (e.g. {@code "h"}, {@code "m"})
   * @return a JSON string describing the ambiguity, or {@code null}
   * @throws RuntimeException if the native call fails
   */
  public static String extensionAmbiguity(String ext) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cExt = arena.allocateFrom(ext);
      MemorySegment result = (MemorySegment) EXTENSION_AMBIGUITY.invokeExact(cExt);

      if (result.equals(MemorySegment.NULL)) {
        return null;
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_extension_ambiguity", t);
    }
  }

  /**
   * Returns the highlights query for the given language.
   *
   * @param language the language name (e.g. {@code "python"}, {@code "java"})
   * @return the highlights query string, or {@code null} if not available
   * @throws RuntimeException if the native call fails
   */
  public static String getHighlightsQuery(String language) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cLang = arena.allocateFrom(language);
      MemorySegment result = (MemorySegment) GET_HIGHLIGHTS_QUERY.invokeExact(cLang);

      if (result.equals(MemorySegment.NULL)) {
        return null;
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_get_highlights_query", t);
    }
  }

  /**
   * Returns the injections query for the given language.
   *
   * @param language the language name (e.g. {@code "markdown"}, {@code "html"})
   * @return the injections query string, or {@code null} if not available
   * @throws RuntimeException if the native call fails
   */
  public static String getInjectionsQuery(String language) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cLang = arena.allocateFrom(language);
      MemorySegment result = (MemorySegment) GET_INJECTIONS_QUERY.invokeExact(cLang);

      if (result.equals(MemorySegment.NULL)) {
        return null;
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_get_injections_query", t);
    }
  }

  /**
   * Returns the locals query for the given language.
   *
   * @param language the language name (e.g. {@code "python"}, {@code "java"})
   * @return the locals query string, or {@code null} if not available
   * @throws RuntimeException if the native call fails
   */
  public static String getLocalsQuery(String language) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cLang = arena.allocateFrom(language);
      MemorySegment result = (MemorySegment) GET_LOCALS_QUERY.invokeExact(cLang);

      if (result.equals(MemorySegment.NULL)) {
        return null;
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_get_locals_query", t);
    }
  }

  /**
   * Extracts patterns from source code using a JSON extraction configuration.
   *
   * <p>The {@code configJson} parameter is a JSON string with fields:
   *
   * <ul>
   *   <li>{@code "language"} (string, required): the language name
   *   <li>{@code "patterns"} (object, required): named patterns to extract
   * </ul>
   *
   * @param source the source code to extract from
   * @param configJson a JSON string specifying the extraction configuration
   * @return a JSON string containing the extraction results
   * @throws RuntimeException if extraction fails
   */
  public static String extract(String source, String configJson) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cSource = arena.allocateFrom(source);
      MemorySegment cConfig = arena.allocateFrom(configJson);
      MemorySegment result =
          (MemorySegment)
              EXTRACT.invokeExact(
                  cSource,
                  (long) source.getBytes(java.nio.charset.StandardCharsets.UTF_8).length,
                  cConfig);

      if (result.equals(MemorySegment.NULL)) {
        String error = lastError();
        throw new RuntimeException(
            "ts_pack_extract returned null" + (error != null ? ": " + error : ""));
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_extract", t);
    }
  }

  /**
   * Validates extraction patterns without running them.
   *
   * <p>The {@code configJson} parameter has the same shape as for {@link #extract(String, String)}.
   *
   * @param configJson a JSON string specifying the extraction configuration to validate
   * @return a JSON string containing the validation results
   * @throws RuntimeException if validation fails
   */
  public static String validateExtraction(String configJson) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cConfig = arena.allocateFrom(configJson);
      MemorySegment result = (MemorySegment) VALIDATE_EXTRACTION.invokeExact(cConfig);

      if (result.equals(MemorySegment.NULL)) {
        String error = lastError();
        throw new RuntimeException(
            "ts_pack_validate_extraction returned null" + (error != null ? ": " + error : ""));
      }

      try {
        return result.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(result);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_validate_extraction", t);
    }
  }

  /** Clears the last error on the current thread. */
  public static void clearError() {
    try {
      CLEAR_ERROR.invokeExact();
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_clear_error", t);
    }
  }

  // --- internal helpers ---

  /**
   * Reads the last error message from the FFI layer (thread-local storage).
   *
   * <p>The returned pointer is valid only until the next FFI call on the same thread, so callers
   * must copy the string immediately.
   *
   * @return the error message, or {@code null} if no error is pending
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
      LOGGER.log(System.Logger.Level.WARNING, "Failed to read FFI error message", t);
      return null;
    }
  }

  /**
   * Initializes the language pack with configuration (static method).
   *
   * <p>{@code configJson} is a JSON string with optional fields:
   *
   * <ul>
   *   <li>{@code "cache_dir"} (string): override default cache directory
   *   <li>{@code "languages"} (array): languages to pre-download
   *   <li>{@code "groups"} (array): language groups to pre-download
   * </ul>
   *
   * @param configJson a JSON string specifying the configuration (may be null or empty)
   * @throws RuntimeException if initialization fails
   */
  public static void init(String configJson) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cConfig =
          configJson != null && !configJson.isEmpty()
              ? arena.allocateFrom(configJson)
              : MemorySegment.NULL;
      int rc = (int) INIT.invokeExact(cConfig);
      if (rc != 0) {
        String error = lastError();
        throw new RuntimeException("ts_pack_init failed" + (error != null ? ": " + error : ""));
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_init", t);
    }
  }

  /**
   * Configures the language pack cache directory without downloading (static method).
   *
   * <p>{@code configJson} is a JSON string with optional fields:
   *
   * <ul>
   *   <li>{@code "cache_dir"} (string): override default cache directory
   * </ul>
   *
   * @param configJson a JSON string specifying the configuration (may be null or empty)
   * @throws RuntimeException if configuration fails
   */
  public static void configure(String configJson) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cConfig =
          configJson != null && !configJson.isEmpty()
              ? arena.allocateFrom(configJson)
              : MemorySegment.NULL;
      int rc = (int) CONFIGURE.invokeExact(cConfig);
      if (rc != 0) {
        String error = lastError();
        throw new RuntimeException(
            "ts_pack_configure failed" + (error != null ? ": " + error : ""));
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_configure", t);
    }
  }

  /**
   * Downloads specific languages to the cache (static method).
   *
   * @param languages a list of language names to download
   * @return the number of newly downloaded languages
   * @throws RuntimeException if the download fails
   */
  public static int download(List<String> languages) {
    if (languages == null || languages.isEmpty()) {
      return 0;
    }

    try (Arena arena = Arena.ofConfined()) {
      MemorySegment namesArray = arena.allocate(ValueLayout.ADDRESS.byteSize() * languages.size());
      for (int i = 0; i < languages.size(); i++) {
        MemorySegment name = arena.allocateFrom(languages.get(i));
        namesArray.setAtIndex(ValueLayout.ADDRESS, i, name);
      }

      int rc = (int) DOWNLOAD.invokeExact(namesArray, (long) languages.size());
      if (rc < 0) {
        String error = lastError();
        throw new RuntimeException("ts_pack_download failed" + (error != null ? ": " + error : ""));
      }
      return rc;
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_download", t);
    }
  }

  /**
   * Downloads all available languages from the remote manifest (static method).
   *
   * @return the number of newly downloaded languages
   * @throws RuntimeException if the download fails
   */
  public static int downloadAll() {
    try {
      int rc = (int) DOWNLOAD_ALL.invokeExact();
      if (rc < 0) {
        String error = lastError();
        throw new RuntimeException(
            "ts_pack_download_all failed" + (error != null ? ": " + error : ""));
      }
      return rc;
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_download_all", t);
    }
  }

  /**
   * Gets all language names available in the remote manifest (static method).
   *
   * @return an unmodifiable list of language names
   * @throws RuntimeException if the operation fails
   */
  public static List<String> manifestLanguages() {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment countPtr = arena.allocate(ValueLayout.JAVA_LONG);
      MemorySegment arr = (MemorySegment) MANIFEST_LANGUAGES.invokeExact(countPtr);

      if (arr.equals(MemorySegment.NULL)) {
        String error = lastError();
        throw new RuntimeException(
            "ts_pack_manifest_languages failed" + (error != null ? ": " + error : ""));
      }

      try {
        long count = countPtr.get(ValueLayout.JAVA_LONG, 0);
        List<String> languages = new ArrayList<>((int) count);
        for (int i = 0; i < count; i++) {
          MemorySegment strPtr = arr.getAtIndex(ValueLayout.ADDRESS, i);
          String name = strPtr.reinterpret(Long.MAX_VALUE).getString(0);
          languages.add(name);
          // Free each individual string before freeing the array
          FREE_STRING.invokeExact(strPtr);
        }
        return Collections.unmodifiableList(languages);
      } finally {
        FREE_STRING_ARRAY.invokeExact(arr);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_manifest_languages", t);
    }
  }

  /**
   * Gets all languages that are already downloaded and cached locally (static method).
   *
   * @return an unmodifiable list of locally cached language names
   * @throws RuntimeException if the operation fails
   */
  public static List<String> downloadedLanguages() {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment countPtr = arena.allocate(ValueLayout.JAVA_LONG);
      MemorySegment arr = (MemorySegment) DOWNLOADED_LANGUAGES.invokeExact(countPtr);

      if (arr.equals(MemorySegment.NULL)) {
        return Collections.emptyList();
      }

      try {
        long count = countPtr.get(ValueLayout.JAVA_LONG, 0);
        List<String> languages = new ArrayList<>((int) count);
        for (int i = 0; i < count; i++) {
          MemorySegment strPtr = arr.getAtIndex(ValueLayout.ADDRESS, i);
          String name = strPtr.reinterpret(Long.MAX_VALUE).getString(0);
          languages.add(name);
          // Free each individual string before freeing the array
          FREE_STRING.invokeExact(strPtr);
        }
        return Collections.unmodifiableList(languages);
      } finally {
        FREE_STRING_ARRAY.invokeExact(arr);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_downloaded_languages", t);
    }
  }

  /**
   * Deletes all cached parser shared libraries (static method).
   *
   * @throws RuntimeException if the operation fails
   */
  public static void cleanCache() {
    try {
      int rc = (int) CLEAN_CACHE.invokeExact();
      if (rc != 0) {
        String error = lastError();
        throw new RuntimeException(
            "ts_pack_clean_cache failed" + (error != null ? ": " + error : ""));
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_clean_cache", t);
    }
  }

  /**
   * Gets the effective cache directory path (static method).
   *
   * @return the cache directory path as a string
   * @throws RuntimeException if the operation fails
   */
  public static String cacheDir() {
    try {
      MemorySegment cStr = (MemorySegment) CACHE_DIR.invokeExact();
      if (cStr.equals(MemorySegment.NULL)) {
        String error = lastError();
        throw new RuntimeException(
            "ts_pack_cache_dir failed" + (error != null ? ": " + error : ""));
      }

      try {
        return cStr.reinterpret(Long.MAX_VALUE).getString(0);
      } finally {
        FREE_STRING.invokeExact(cStr);
      }
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke ts_pack_cache_dir", t);
    }
  }

  /**
   * Returns the current registry pointer, throwing if the registry is closed.
   *
   * @return the non-null registry pointer
   * @throws IllegalStateException if the registry has been closed
   */
  private MemorySegment ensureOpen() {
    MemorySegment ptr = registryPtr.get();
    if (ptr == null || ptr.equals(MemorySegment.NULL)) {
      throw new IllegalStateException("Registry has been closed");
    }
    return ptr;
  }
}
