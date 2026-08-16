package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class DiagnosticSeverityTest {

    @Test
    void shouldExposeThreeVariants() {
        assertEquals(3, DiagnosticSeverity.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Error", DiagnosticSeverity.Error.getValue());
        assertEquals("Warning", DiagnosticSeverity.Warning.getValue());
        assertEquals("Info", DiagnosticSeverity.Info.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(DiagnosticSeverity.Warning, DiagnosticSeverity.fromValue("WARNING"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> DiagnosticSeverity.fromValue("Critical"));
    }
}
