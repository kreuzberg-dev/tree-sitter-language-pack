package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class DataNodeKindTest {

    @Test
    void shouldExposeThreeVariants() {
        assertEquals(3, DataNodeKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("KeyValue", DataNodeKind.KeyValue.getValue());
        assertEquals("Element", DataNodeKind.Element.getValue());
        assertEquals("Sequence", DataNodeKind.Sequence.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(DataNodeKind.Sequence, DataNodeKind.fromValue("sequence"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> DataNodeKind.fromValue("Bogus"));
    }
}
