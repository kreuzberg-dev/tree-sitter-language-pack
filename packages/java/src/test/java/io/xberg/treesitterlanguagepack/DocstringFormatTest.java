package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class DocstringFormatTest {

    @Test
    void shouldExposeSixVariants() {
        assertEquals(6, DocstringFormat.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("PythonTripleQuote", DocstringFormat.PythonTripleQuote.getValue());
        assertEquals("JSDoc", DocstringFormat.JSDoc.getValue());
        assertEquals("Rustdoc", DocstringFormat.Rustdoc.getValue());
        assertEquals("GoDoc", DocstringFormat.GoDoc.getValue());
        assertEquals("JavaDoc", DocstringFormat.JavaDoc.getValue());
        assertEquals("Other", DocstringFormat.Other.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(DocstringFormat.JSDoc, DocstringFormat.fromValue("jsdoc"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> DocstringFormat.fromValue("Doxygen"));
    }
}
