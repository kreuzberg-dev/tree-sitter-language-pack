package io.github.treesitter.languagepack;

import static org.junit.jupiter.api.Assertions.*;

import dev.kreuzberg.tslp.TreeSitterLanguagePack;
import java.util.List;
import org.junit.jupiter.api.Test;

/**
 * Tests for {@link TreeSitterLanguagePack}.
 *
 * <p>These tests require the {@code ts_pack_ffi} native library to be available. Set the {@code
 * TSPACK_LIB_PATH} environment variable to the full path of the shared library if it is not on the
 * system library path.
 */
class TsPackRegistryTest {

  @Test
  void testLanguageCount() {
    long count = TreeSitterLanguagePack.languageCount();
    assertTrue(count > 0, "Expected at least one language, got " + count);
  }

  @Test
  void testAvailableLanguages() {
    List<String> languages = TreeSitterLanguagePack.availableLanguages();
    assertNotNull(languages);
    assertFalse(languages.isEmpty(), "Available languages list should not be empty");
    assertEquals(
        TreeSitterLanguagePack.languageCount(),
        languages.size(),
        "availableLanguages size should match languageCount");
  }

  @Test
  void testHasLanguageKnown() {
    List<String> languages = TreeSitterLanguagePack.availableLanguages();
    assertFalse(languages.isEmpty(), "Expected at least one language to be available");
    String firstName = languages.get(0);
    assertTrue(
        TreeSitterLanguagePack.hasLanguage(firstName),
        "hasLanguage should return true for: " + firstName);
  }

  @Test
  void testHasLanguageUnknown() {
    assertFalse(
        TreeSitterLanguagePack.hasLanguage("__nonexistent_language_42__"),
        "hasLanguage should return false for a made-up language name");
  }

  @Test
  void testAvailableLanguagesContents() {
    List<String> languages = TreeSitterLanguagePack.availableLanguages();
    for (String lang : languages) {
      assertNotNull(lang, "Language name should not be null");
      assertFalse(lang.isEmpty(), "Language name should not be empty");
    }
  }

  @Test
  void testDetectLanguageFromExtension() {
    String language = TreeSitterLanguagePack.detectLanguageFromExtension("py");
    assertNotNull(language, "Expected a language for extension 'py'");
    assertFalse(language.isEmpty(), "Detected language should not be empty");
  }

  @Test
  void testDetectLanguageFromPath() {
    String language = TreeSitterLanguagePack.detectLanguageFromPath("main.rs");
    assertNotNull(language, "Expected a language for path 'main.rs'");
    assertFalse(language.isEmpty(), "Detected language should not be empty");
  }
}
