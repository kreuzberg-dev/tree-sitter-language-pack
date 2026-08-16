package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class StructureKindTest {

    @Test
    void shouldExposeElevenVariants() {
        assertEquals(11, StructureKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Function", StructureKind.Function.getValue());
        assertEquals("Method", StructureKind.Method.getValue());
        assertEquals("Class", StructureKind.Class.getValue());
        assertEquals("Struct", StructureKind.Struct.getValue());
        assertEquals("Interface", StructureKind.Interface.getValue());
        assertEquals("Enum", StructureKind.Enum.getValue());
        assertEquals("Module", StructureKind.Module.getValue());
        assertEquals("Trait", StructureKind.Trait.getValue());
        assertEquals("Impl", StructureKind.Impl.getValue());
        assertEquals("Namespace", StructureKind.Namespace.getValue());
        assertEquals("Other", StructureKind.Other.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(StructureKind.Namespace, StructureKind.fromValue("namespace"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> StructureKind.fromValue("Macro"));
    }
}
