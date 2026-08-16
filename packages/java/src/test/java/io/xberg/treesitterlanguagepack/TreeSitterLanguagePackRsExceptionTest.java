package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;

import org.junit.jupiter.api.Test;

class TreeSitterLanguagePackRsExceptionTest {

    @Test
    void shouldExposeCodeAndMessagePassedToCodeConstructor() {
        TreeSitterLanguagePackRsException exception = new TreeSitterLanguagePackRsException(7, "boom");

        assertEquals(7, exception.getCode());
        assertEquals("boom", exception.getMessage());
    }

    @Test
    void shouldDefaultCodeToMinusOneWhenConstructedWithCause() {
        Throwable cause = new RuntimeException("underlying");

        TreeSitterLanguagePackRsException exception = new TreeSitterLanguagePackRsException("wrapped", cause);

        assertEquals(-1, exception.getCode());
        assertEquals("wrapped", exception.getMessage());
        assertSame(cause, exception.getCause());
    }
}
