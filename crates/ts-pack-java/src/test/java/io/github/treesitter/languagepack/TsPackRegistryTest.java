package io.github.treesitter.languagepack;

import org.junit.jupiter.api.Test;

import java.lang.foreign.MemorySegment;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests for {@link TsPackRegistry}.
 *
 * <p>These tests require the {@code ts_pack_ffi} native library to be available.
 * Set the {@code TSPACK_LIB_PATH} environment variable to the full path of the
 * shared library if it is not on the system library path.</p>
 */
class TsPackRegistryTest {

    @Test
    void testCreateAndClose() {
        TsPackRegistry registry = new TsPackRegistry();
        assertNotNull(registry);
        // Should not throw
        registry.close();
        // Double close should be safe
        registry.close();
    }

    @Test
    void testCreateWithTryWithResources() {
        assertDoesNotThrow(() -> {
            try (var registry = new TsPackRegistry()) {
                assertNotNull(registry);
            }
        });
    }

    @Test
    void testLanguageCount() {
        try (var registry = new TsPackRegistry()) {
            int count = registry.languageCount();
            assertTrue(count > 0, "Expected at least one language, got " + count);
        }
    }

    @Test
    void testAvailableLanguages() {
        try (var registry = new TsPackRegistry()) {
            List<String> languages = registry.availableLanguages();
            assertNotNull(languages);
            assertFalse(languages.isEmpty(), "Available languages list should not be empty");
            // The list should contain well-known languages
            assertTrue(languages.size() == registry.languageCount(),
                    "availableLanguages size should match languageCount");
        }
    }

    @Test
    void testHasLanguage() {
        try (var registry = new TsPackRegistry()) {
            // Pick the first available language — it must exist
            String firstName = registry.languageNameAt(0);
            assertTrue(registry.hasLanguage(firstName),
                    "Registry should contain language: " + firstName);

            assertFalse(registry.hasLanguage("__nonexistent_language_42__"),
                    "Registry should not contain a made-up language name");
        }
    }

    @Test
    void testGetLanguage() {
        try (var registry = new TsPackRegistry()) {
            // Use the first available language for a reliable test
            String firstName = registry.languageNameAt(0);
            MemorySegment lang = registry.getLanguage(firstName);
            assertNotNull(lang);
            assertNotEquals(MemorySegment.NULL, lang,
                    "getLanguage should return a non-null pointer for: " + firstName);
        }
    }

    @Test
    void testGetLanguageNotFound() {
        try (var registry = new TsPackRegistry()) {
            assertThrows(IllegalArgumentException.class, () ->
                    registry.getLanguage("__nonexistent_language_42__")
            );
        }
    }

    @Test
    void testLanguageNameAtOutOfBounds() {
        try (var registry = new TsPackRegistry()) {
            int count = registry.languageCount();
            assertThrows(IndexOutOfBoundsException.class, () ->
                    registry.languageNameAt(count)
            );
            assertThrows(IndexOutOfBoundsException.class, () ->
                    registry.languageNameAt(-1)
            );
        }
    }

    @Test
    void testUseAfterClose() {
        TsPackRegistry registry = new TsPackRegistry();
        registry.close();

        assertThrows(IllegalStateException.class, registry::languageCount);
        assertThrows(IllegalStateException.class, () -> registry.hasLanguage("java"));
        assertThrows(IllegalStateException.class, () -> registry.getLanguage("java"));
        assertThrows(IllegalStateException.class, registry::availableLanguages);
        assertThrows(IllegalStateException.class, () -> registry.languageNameAt(0));
    }

    @Test
    void testClearError() {
        try (var registry = new TsPackRegistry()) {
            // Trigger an error by requesting a nonexistent language
            assertThrows(IllegalArgumentException.class, () ->
                    registry.getLanguage("__nonexistent_language_42__")
            );

            // Clear should not throw
            assertDoesNotThrow(TsPackRegistry::clearError);
        }
    }

    @Test
    void testAvailableLanguagesContents() {
        try (var registry = new TsPackRegistry()) {
            List<String> languages = registry.availableLanguages();
            // Each entry should be a non-empty string
            for (String lang : languages) {
                assertNotNull(lang);
                assertFalse(lang.isEmpty(), "Language name should not be empty");
            }
            // The returned list should be unmodifiable
            assertThrows(UnsupportedOperationException.class, () -> languages.add("bogus"));
        }
    }
}
