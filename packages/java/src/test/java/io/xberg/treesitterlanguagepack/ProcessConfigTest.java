package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class ProcessConfigTest {

    @Test
    void shouldDefaultStructureImportsAndExportsToTrueWhenNull() {
        ProcessConfig config = new ProcessConfig(
            "python", null, null, null, null, null, null, null, null, null, null, null
        );

        assertTrue(config.structure());
        assertTrue(config.imports());
        assertTrue(config.exports());
    }

    @Test
    void shouldPreserveExplicitFalseForStructureImportsAndExports() {
        ProcessConfig config = new ProcessConfig(
            "python", false, false, false, null, null, null, null, null, null, null, null
        );

        assertFalse(config.structure());
        assertFalse(config.imports());
        assertFalse(config.exports());
    }

    @Test
    void shouldLeaveOtherOptionalFieldsNullWhenUnset() {
        ProcessConfig config = new ProcessConfig(
            "python", null, null, null, null, null, null, null, null, null, null, null
        );

        assertNull(config.comments());
        assertNull(config.docstrings());
        assertNull(config.symbols());
        assertNull(config.diagnostics());
        assertNull(config.chunkMaxSize());
        assertNull(config.dataExtraction());
        assertNull(config.maxSourceBytes());
        assertNull(config.parseTimeoutMs());
    }

    @Test
    void shouldApplyDefaultsWhenBuiltThroughBuilderWithoutSettingThem() {
        ProcessConfig built = ProcessConfig.builder().withLanguage("go").build();

        assertTrue(built.structure());
        assertTrue(built.imports());
        assertTrue(built.exports());
    }

    @Test
    void shouldRoundTripThroughJsonPreservingExplicitFalseFlags() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        ProcessConfig config = new ProcessConfig(
            "rust", false, true, true, true, true, true, true, 1000L, true, 500000L, 5000L
        );

        String json = mapper.writeValueAsString(config);
        ProcessConfig parsed = mapper.readValue(json, ProcessConfig.class);

        assertEquals(config, parsed);
        assertFalse(parsed.structure());
    }
}
