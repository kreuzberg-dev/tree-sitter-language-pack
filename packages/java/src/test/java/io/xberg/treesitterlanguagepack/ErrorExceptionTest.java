package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class ErrorExceptionTest {

    @Test
    void shouldExposeMessagePassedToSingleArgConstructor() {
        ErrorException exception = new ErrorException("boom");

        assertEquals("boom", exception.getMessage());
        assertNull(exception.getCause());
    }

    @Test
    void shouldExposeMessageAndCausePassedToTwoArgConstructor() {
        Throwable cause = new IllegalStateException("root cause");

        ErrorException exception = new ErrorException("wrapped", cause);

        assertEquals("wrapped", exception.getMessage());
        assertSame(cause, exception.getCause());
    }

    @Test
    void shouldBeACheckedException() {
        assertTrue(Exception.class.isAssignableFrom(ErrorException.class));
        assertFalse(RuntimeException.class.isAssignableFrom(ErrorException.class));
    }
}
