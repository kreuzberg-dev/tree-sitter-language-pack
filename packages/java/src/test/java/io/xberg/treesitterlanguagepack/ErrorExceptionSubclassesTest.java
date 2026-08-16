package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertSame;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.util.List;
import java.util.stream.Stream;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.MethodSource;

/**
 * Every direct {@link ErrorException} subclass shares the same generated shape: a
 * message-only constructor and a message+cause constructor. Verifying that shape once,
 * per class, catches a broken subclass without duplicating the same four assertions
 * thirteen times over.
 */
class ErrorExceptionSubclassesTest {

    private static final List<Class<? extends ErrorException>> SUBCLASSES = List.of(
        CacheLockException.class,
        ChecksumMismatchException.class,
        ConfigException.class,
        DownloadException.class,
        DynamicLoadException.class,
        InvalidRangeException.class,
        LanguageNotFoundException.class,
        LockPoisonedException.class,
        NullLanguagePointerException.class,
        ParseFailedException.class,
        ParserSetupException.class,
        ParseTimeoutException.class,
        QueryErrorException.class
    );

    private static Stream<Class<? extends ErrorException>> subclasses() {
        return SUBCLASSES.stream();
    }

    @ParameterizedTest
    @MethodSource("subclasses")
    void shouldExposeMessagePassedToSingleArgConstructor(final Class<? extends ErrorException> exceptionClass)
            throws Exception {
        Constructor<? extends ErrorException> constructor = exceptionClass.getDeclaredConstructor(String.class);

        ErrorException exception = constructor.newInstance("boom: " + exceptionClass.getSimpleName());

        assertEquals("boom: " + exceptionClass.getSimpleName(), exception.getMessage());
        assertTrue(exception instanceof ErrorException);
    }

    @ParameterizedTest
    @MethodSource("subclasses")
    void shouldExposeMessageAndCausePassedToTwoArgConstructor(final Class<? extends ErrorException> exceptionClass)
            throws Exception {
        Constructor<? extends ErrorException> constructor =
            exceptionClass.getDeclaredConstructor(String.class, Throwable.class);
        Throwable cause = new RuntimeException("root cause for " + exceptionClass.getSimpleName());

        ErrorException exception = constructor.newInstance("wrapped", cause);

        assertEquals("wrapped", exception.getMessage());
        assertSame(cause, exception.getCause());
    }

    @ParameterizedTest
    @MethodSource("subclasses")
    void shouldHaveExactlyOneSerialVersionUidConstant(final Class<? extends ErrorException> exceptionClass)
            throws Exception {
        Field field = exceptionClass.getDeclaredField("serialVersionUID");
        field.setAccessible(true);

        assertEquals(1L, field.getLong(null));
    }
}
