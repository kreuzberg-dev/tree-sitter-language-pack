package dev.kreuzberg.tslp;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.nio.file.Path;
import java.util.Collections;
import java.util.List;

/**
 * Static facade over the {@code ts_pack_ffi} native library using the new {@code tslp_*} symbols.
 *
 * <p>The native library is resolved from the {@code TSPACK_LIB_PATH} environment variable (full
 * path to the shared library) or from the system library path under the name {@code ts_pack_ffi}.
 */
public final class TreeSitterLanguagePack {

  private static final Linker LINKER = Linker.nativeLinker();
  private static final SymbolLookup LOOKUP;
  private static final MethodHandle AVAILABLE_LANGUAGES;
  private static final MethodHandle HAS_LANGUAGE;
  private static final MethodHandle LANGUAGE_COUNT;
  private static final MethodHandle DETECT_LANGUAGE_FROM_EXTENSION;
  private static final MethodHandle DETECT_LANGUAGE_FROM_PATH;
  private static final MethodHandle FREE_STRING;

  static {
    String libPath = System.getenv("TSPACK_LIB_PATH");
    if (libPath != null && !libPath.isEmpty()) {
      LOOKUP = SymbolLookup.libraryLookup(Path.of(libPath), Arena.global());
    } else {
      LOOKUP = SymbolLookup.libraryLookup("ts_pack_ffi", Arena.global());
    }

    AVAILABLE_LANGUAGES =
        LINKER.downcallHandle(
            LOOKUP.find("tslp_available_languages").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS));

    HAS_LANGUAGE =
        LINKER.downcallHandle(
            LOOKUP.find("tslp_has_language").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS));

    LANGUAGE_COUNT =
        LINKER.downcallHandle(
            LOOKUP.find("tslp_language_count").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.JAVA_LONG));

    DETECT_LANGUAGE_FROM_EXTENSION =
        LINKER.downcallHandle(
            LOOKUP.find("tslp_detect_language_from_extension").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    DETECT_LANGUAGE_FROM_PATH =
        LINKER.downcallHandle(
            LOOKUP.find("tslp_detect_language_from_path").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS));

    FREE_STRING =
        LINKER.downcallHandle(
            LOOKUP.find("tslp_free_string").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS));
  }

  private TreeSitterLanguagePack() {}

  /**
   * Returns the number of languages available in the pack.
   *
   * @return language count
   */
  public static long languageCount() {
    try {
      return (long) LANGUAGE_COUNT.invokeExact();
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke tslp_language_count", t);
    }
  }

  /**
   * Returns the names of all languages available in the pack.
   *
   * <p>The list is deserialized from the JSON string returned by the native library.
   *
   * @return unmodifiable list of language names
   */
  public static List<String> availableLanguages() {
    MemorySegment ptr;
    try {
      ptr = (MemorySegment) AVAILABLE_LANGUAGES.invokeExact();
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke tslp_available_languages", t);
    }

    if (ptr == null || ptr.equals(MemorySegment.NULL)) {
      return Collections.emptyList();
    }

    try {
      String json = ptr.reinterpret(Long.MAX_VALUE).getString(0);
      List<String> languages =
          new Gson().fromJson(json, new TypeToken<List<String>>() {}.getType());
      return Collections.unmodifiableList(languages);
    } finally {
      freeString(ptr);
    }
  }

  /**
   * Returns {@code true} if the named language is available in the pack.
   *
   * @param name language name (e.g. {@code "java"}, {@code "python"})
   * @return {@code true} if the language is available
   */
  public static boolean hasLanguage(String name) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cName = arena.allocateFrom(name);
      int result = (int) HAS_LANGUAGE.invokeExact(cName);
      return result != 0;
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke tslp_has_language", t);
    }
  }

  /**
   * Detects the language for the given file extension.
   *
   * @param ext file extension without leading dot (e.g. {@code "py"}, {@code "rs"})
   * @return detected language name, or {@code null} if unrecognised
   */
  public static String detectLanguageFromExtension(String ext) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cExt = arena.allocateFrom(ext);
      MemorySegment ptr = (MemorySegment) DETECT_LANGUAGE_FROM_EXTENSION.invokeExact(cExt);
      return readAndFreeString(ptr);
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke tslp_detect_language_from_extension", t);
    }
  }

  /**
   * Detects the language for the given file path.
   *
   * @param path file path or name (e.g. {@code "main.rs"})
   * @return detected language name, or {@code null} if unrecognised
   */
  public static String detectLanguageFromPath(String path) {
    try (Arena arena = Arena.ofConfined()) {
      MemorySegment cPath = arena.allocateFrom(path);
      MemorySegment ptr = (MemorySegment) DETECT_LANGUAGE_FROM_PATH.invokeExact(cPath);
      return readAndFreeString(ptr);
    } catch (RuntimeException e) {
      throw e;
    } catch (Throwable t) {
      throw new RuntimeException("Failed to invoke tslp_detect_language_from_path", t);
    }
  }

  private static String readAndFreeString(MemorySegment ptr) {
    if (ptr == null || ptr.equals(MemorySegment.NULL)) {
      return null;
    }
    try {
      return ptr.reinterpret(Long.MAX_VALUE).getString(0);
    } finally {
      freeString(ptr);
    }
  }

  private static void freeString(MemorySegment ptr) {
    try {
      FREE_STRING.invokeExact(ptr);
    } catch (Throwable t) {
      // Best-effort free; do not mask the original result
    }
  }
}
