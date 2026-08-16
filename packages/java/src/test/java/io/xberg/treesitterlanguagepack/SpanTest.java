package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class SpanTest {

    @Test
    void shouldExposeAllSixAccessors() {
        Span span = new Span(1, 2, 3, 4, 5, 6);

        assertEquals(1, span.startByte());
        assertEquals(2, span.endByte());
        assertEquals(3, span.startLine());
        assertEquals(4, span.startColumn());
        assertEquals(5, span.endLine());
        assertEquals(6, span.endColumn());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        Span built = Span.builder()
            .withStartByte(0)
            .withEndByte(10)
            .withStartLine(1)
            .withStartColumn(0)
            .withEndLine(1)
            .withEndColumn(10)
            .build();

        assertEquals(new Span(0, 10, 1, 0, 1, 10), built);
    }

    @Test
    void shouldRoundTripThroughJsonUsingSnakeCaseKeys() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        Span span = new Span(0, 10, 1, 0, 1, 10);

        String json = mapper.writeValueAsString(span);
        Span parsed = mapper.readValue(json, Span.class);

        assertEquals(span, parsed);
        assertEquals(
            "{\"start_byte\":0,\"end_byte\":10,\"start_line\":1,\"start_column\":0,\"end_line\":1,\"end_column\":10}",
            json
        );
    }
}
