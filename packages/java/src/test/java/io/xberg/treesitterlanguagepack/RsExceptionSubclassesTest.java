package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class RsExceptionSubclassesTest {

    @Test
    void shouldAssignFixedCodeTwoToConversionErrorExceptionMessageConstructor() {
        ConversionErrorException exception = new ConversionErrorException("bad conversion");

        assertEquals(2, exception.getCode());
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

    @Test
    void shouldAssignFixedCodeOneToInvalidInputExceptionMessageConstructor() {
        InvalidInputException exception = new InvalidInputException("bad input");

        assertEquals(1, exception.getCode());
        assertEquals("bad input", exception.getMessage());
        assertTrue(exception instanceof TreeSitterLanguagePackRsException);
    }

    @Test
    void shouldFallBackToDefaultCodeWhenInvalidInputExceptionIsConstructedWithCause() {
        Throwable cause = new RuntimeException("root");

        InvalidInputException exception = new InvalidInputException("bad input", cause);

        assertEquals(-1, exception.getCode());
        assertSame(cause, exception.getCause());
    }
}
