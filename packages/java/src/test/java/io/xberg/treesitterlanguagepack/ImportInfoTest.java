package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;
import org.junit.jupiter.api.Test;

class ImportInfoTest {

    private static final Span SAMPLE_SPAN = new Span(0, 12, 0, 0, 0, 12);

    @Test
    void shouldExposeAllAccessorsForNamedImport() {
        ImportInfo importInfo = new ImportInfo("os", List.of("path", "getcwd"), null, false, SAMPLE_SPAN);

        assertEquals("os", importInfo.source());
        assertEquals(List.of("path", "getcwd"), importInfo.items());
        assertNull(importInfo.alias());
        assertFalse(importInfo.isWildcard());
        assertEquals(SAMPLE_SPAN, importInfo.span());
    }

    @Test
    void shouldSupportWildcardImportWithAlias() {
        ImportInfo importInfo = new ImportInfo("numpy", null, "np", true, SAMPLE_SPAN);

        assertEquals("np", importInfo.alias());
        assertTrue(importInfo.isWildcard());
        assertEquals(List.of(), importInfo.items());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        ImportInfo built = ImportInfo.builder()
            .withSource("sys")
            .withItems(List.of("argv"))
            .withAlias(null)
            .withIsWildcard(false)
            .withSpan(SAMPLE_SPAN)
            .build();

        assertEquals(new ImportInfo("sys", List.of("argv"), null, false, SAMPLE_SPAN), built);
    }

    @Test
    void shouldRoundTripThroughJsonUsingSnakeCaseIsWildcardKey() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        ImportInfo importInfo = new ImportInfo("json", null, null, true, SAMPLE_SPAN);

        String json = mapper.writeValueAsString(importInfo);

        assertTrue(json.contains("\"is_wildcard\":true"));
        assertEquals(importInfo, mapper.readValue(json, ImportInfo.class));
    }
}
