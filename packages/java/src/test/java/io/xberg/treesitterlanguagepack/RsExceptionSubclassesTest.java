package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class RsExceptionSubclassesTest {

    @Test
    void shouldAssignFixedCodeOneToConversionErrorExceptionMessageConstructor() {
        ConversionErrorException exception = new ConversionErrorException("bad conversion");

        assertEquals(1, exception.getCode());
        assertEquals("bad conversion", exception.getMessage());
        assertTrue(exception instanceof TreeSitterLanguagePackRsException);
    }

    @Test
    void shouldFallBackToDefaultCodeWhenConversionErrorExceptionIsConstructedWithCause() {
        Throwable cause = new RuntimeException("root");

        ConversionErrorException exception = new ConversionErrorException("bad conversion", cause);

        assertEquals(-1, exception.getCode());
        assertSame(cause, exception.getCause());
    }
}
