package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class ExportKindTest {

    @Test
    void shouldExposeThreeVariants() {
        assertEquals(3, ExportKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Named", ExportKind.Named.getValue());
        assertEquals("Default", ExportKind.Default.getValue());
        assertEquals("ReExport", ExportKind.ReExport.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(ExportKind.ReExport, ExportKind.fromValue("reexport"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> ExportKind.fromValue("Star"));
    }
}
