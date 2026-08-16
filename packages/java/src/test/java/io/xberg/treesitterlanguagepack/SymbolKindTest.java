package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class SymbolKindTest {

    @Test
    void shouldExposeNineVariants() {
        assertEquals(9, SymbolKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Variable", SymbolKind.Variable.getValue());
        assertEquals("Constant", SymbolKind.Constant.getValue());
        assertEquals("Function", SymbolKind.Function.getValue());
        assertEquals("Class", SymbolKind.Class.getValue());
        assertEquals("Type", SymbolKind.Type.getValue());
        assertEquals("Interface", SymbolKind.Interface.getValue());
        assertEquals("Enum", SymbolKind.Enum.getValue());
        assertEquals("Module", SymbolKind.Module.getValue());
        assertEquals("Other", SymbolKind.Other.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(SymbolKind.Function, SymbolKind.fromValue("FUNCTION"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> SymbolKind.fromValue("Unknown"));
    }
}
