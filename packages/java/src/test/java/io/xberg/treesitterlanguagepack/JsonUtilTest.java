package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class JsonUtilTest {

    @Test
    void shouldDeserializeSnakeCaseJsonIntoRecordUsingItsBuilder() throws Exception {
        Point point = JsonUtil.fromJson("{\"row\":2,\"column\":9}", Point.class);

        assertEquals(new Point(2, 9), point);
    }

    @Test
    void shouldWrapMalformedJsonInTreeSitterLanguagePackRsException() {
        TreeSitterLanguagePackRsException exception = assertThrows(
            TreeSitterLanguagePackRsException.class,
            () -> JsonUtil.fromJson("{not valid json", Point.class)
        );

        assertTrue(exception.getMessage().startsWith("Failed to parse Point from JSON:"));
    }

    @Test
    void shouldWrapTypeMismatchInTreeSitterLanguagePackRsException() {
        assertThrows(
            TreeSitterLanguagePackRsException.class,
            () -> JsonUtil.fromJson("{\"row\":\"not-a-number\",\"column\":1}", Point.class)
        );
    }
}
