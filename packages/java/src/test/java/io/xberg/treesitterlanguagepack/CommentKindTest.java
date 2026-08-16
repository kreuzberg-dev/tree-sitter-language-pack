package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class CommentKindTest {

    @Test
    void shouldExposeThreeVariants() {
        assertEquals(3, CommentKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Line", CommentKind.Line.getValue());
        assertEquals("Block", CommentKind.Block.getValue());
        assertEquals("Doc", CommentKind.Doc.getValue());
    }

    @Test
    void shouldReturnWireFormatValueFromToString() {
        assertEquals("Line", CommentKind.Line.toString());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(CommentKind.Block, CommentKind.fromValue("block"));
        assertEquals(CommentKind.Block, CommentKind.fromValue("BLOCK"));
        assertEquals(CommentKind.Block, CommentKind.fromValue("Block"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        IllegalArgumentException exception = assertThrows(
            IllegalArgumentException.class, () -> CommentKind.fromValue("NotAKind")
        );

        assertEquals("Unknown CommentKind value: NotAKind", exception.getMessage());
    }
}
