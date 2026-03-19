package dev.kreuzberg.test;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;
import dev.kreuzberg.TreeSitterLanguagePack;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;

import java.io.IOException;
import java.lang.reflect.Type;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Map;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

class SmokeTest {

    private static final Gson GSON = new Gson();
    private static final Path FIXTURES_DIR = Path.of("..", "fixtures");

    @BeforeAll
    static void setup() {
        // Download required languages
        TreeSitterLanguagePack.download(List.of("python", "javascript", "rust", "go", "ruby", "java", "c", "cpp"));
    }

    @SuppressWarnings("unchecked")
    private static List<Map<String, Object>> loadFixtures(String name) throws IOException {
        String json = Files.readString(FIXTURES_DIR.resolve(name));
        Type type = new TypeToken<List<Map<String, Object>>>() {}.getType();
        return GSON.fromJson(json, type);
    }

    // Basic fixtures tests
    @TestFactory
    Stream<DynamicTest> basicFixtures() throws IOException {
        List<Map<String, Object>> fixtures = loadFixtures("basic.json");

        return fixtures.stream().map(fixture -> DynamicTest.dynamicTest(
            (String) fixture.get("name"),
            () -> {
                String test = (String) fixture.get("test");
                switch (test) {
                    case "language_count" -> {
                        int count = TreeSitterLanguagePack.languageCount();
                        int expectedMin = ((Number) fixture.get("expected_min")).intValue();
                        assertTrue(count >= expectedMin,
                            "language_count " + count + " < expected min " + expectedMin);
                    }
                    case "has_language" -> {
                        String language = (String) fixture.get("language");
                        boolean result = TreeSitterLanguagePack.hasLanguage(language);
                        boolean expected = (Boolean) fixture.get("expected");
                        assertEquals(expected, result,
                            "has_language(" + language + ") = " + result + ", expected " + expected);
                    }
                    case "available_languages" -> {
                        List<String> langs = TreeSitterLanguagePack.availableLanguages();
                        @SuppressWarnings("unchecked")
                        List<String> expectedContains = (List<String>) fixture.get("expected_contains");
                        for (String lang : expectedContains) {
                            assertTrue(langs.contains(lang),
                                "available_languages missing '" + lang + "'");
                        }
                    }
                    default -> fail("Unknown test type: " + test);
                }
            }
        ));
    }

    // Process fixtures tests
    @TestFactory
    Stream<DynamicTest> processFixtures() throws IOException {
        List<Map<String, Object>> fixtures = loadFixtures("process.json");

        return fixtures.stream().map(fixture -> DynamicTest.dynamicTest(
            (String) fixture.get("name"),
            () -> {
                String source = (String) fixture.get("source");
                @SuppressWarnings("unchecked")
                Map<String, Object> configMap = (Map<String, Object>) fixture.get("config");
                @SuppressWarnings("unchecked")
                Map<String, Object> expected = (Map<String, Object>) fixture.get("expected");

                String configJson = GSON.toJson(configMap);
                Map<String, Object> result = TreeSitterLanguagePack.process(source, configJson);

                if (expected.containsKey("language")) {
                    assertEquals(expected.get("language"), result.get("language"));
                }
                if (expected.containsKey("structure_min")) {
                    @SuppressWarnings("unchecked")
                    List<Object> structure = (List<Object>) result.get("structure");
                    int min = ((Number) expected.get("structure_min")).intValue();
                    assertTrue(structure.size() >= min,
                        "structure count " + structure.size() + " < min " + min);
                }
                if (expected.containsKey("imports_min")) {
                    @SuppressWarnings("unchecked")
                    List<Object> imports = (List<Object>) result.get("imports");
                    int min = ((Number) expected.get("imports_min")).intValue();
                    assertTrue(imports.size() >= min,
                        "imports count " + imports.size() + " < min " + min);
                }
                if (expected.containsKey("error_count")) {
                    @SuppressWarnings("unchecked")
                    Map<String, Object> metrics = (Map<String, Object>) result.get("metrics");
                    int errorCount = ((Number) metrics.get("error_count")).intValue();
                    int expectedCount = ((Number) expected.get("error_count")).intValue();
                    assertEquals(expectedCount, errorCount);
                }
                if (expected.containsKey("metrics_total_lines_min")) {
                    @SuppressWarnings("unchecked")
                    Map<String, Object> metrics = (Map<String, Object>) result.get("metrics");
                    int totalLines = ((Number) metrics.get("total_lines")).intValue();
                    int min = ((Number) expected.get("metrics_total_lines_min")).intValue();
                    assertTrue(totalLines >= min,
                        "total_lines " + totalLines + " < min " + min);
                }
            }
        ));
    }

    // Chunking fixtures tests
    @TestFactory
    Stream<DynamicTest> chunkingFixtures() throws IOException {
        List<Map<String, Object>> fixtures = loadFixtures("chunking.json");

        return fixtures.stream().map(fixture -> DynamicTest.dynamicTest(
            (String) fixture.get("name"),
            () -> {
                String source = (String) fixture.get("source");
                @SuppressWarnings("unchecked")
                Map<String, Object> configMap = (Map<String, Object>) fixture.get("config");
                @SuppressWarnings("unchecked")
                Map<String, Object> expected = (Map<String, Object>) fixture.get("expected");

                String configJson = GSON.toJson(configMap);
                Map<String, Object> result = TreeSitterLanguagePack.process(source, configJson);

                if (expected.containsKey("chunks_min")) {
                    @SuppressWarnings("unchecked")
                    List<Object> chunks = (List<Object>) result.get("chunks");
                    int min = ((Number) expected.get("chunks_min")).intValue();
                    assertTrue(chunks.size() >= min,
                        "chunks count " + chunks.size() + " < min " + min);
                }
            }
        ));
    }

    // Download API tests
    @Test
    void downloadedLanguagesReturnsArray() {
        List<String> langs = TreeSitterLanguagePack.downloadedLanguages();
        assertNotNull(langs);
        assertTrue(langs instanceof List);
    }

    @Test
    void manifestLanguagesReturnsArrayWith50Plus() {
        List<String> langs = TreeSitterLanguagePack.manifestLanguages();
        assertNotNull(langs);
        assertTrue(langs.size() > 50, "manifestLanguages should return 50+ languages");
    }

    @Test
    void cacheDirReturnsNonEmptyString() {
        String dir = TreeSitterLanguagePack.cacheDir();
        assertNotNull(dir);
        assertTrue(dir.length() > 0, "cacheDir should return non-empty string");
    }

    @Test
    void initDoesNotThrow() {
        assertDoesNotThrow(() -> TreeSitterLanguagePack.init());
    }

    // Parse validation tests
    @Test
    void parsesPythonCode() {
        var tree = TreeSitterLanguagePack.parseString("python", "def hello(): pass\n");
        assertNotNull(tree);
        assertEquals("module", tree.rootNodeType());
        assertTrue(tree.rootChildCount() >= 1);
        assertFalse(tree.hasErrorNodes());
    }

    @Test
    void errorsOnInvalidLanguage() {
        assertThrows(Exception.class, () ->
            TreeSitterLanguagePack.parseString("nonexistent_xyz_123", "code"));
    }

    @Test
    void hasLanguageReturnsFalseForNonexistent() {
        boolean result = TreeSitterLanguagePack.hasLanguage("nonexistent_xyz_123");
        assertFalse(result);
    }
}
