package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class ByteRangeTest {

    @Test
    void shouldExposeStartAndEndAccessors() {
        ByteRange range = new ByteRange(10, 20);

        assertEquals(10, range.start());
        assertEquals(20, range.end());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        ByteRange built = ByteRange.builder().withStart(0).withEnd(100).build();

        assertEquals(new ByteRange(0, 100), built);
    }

    @Test
    void shouldRoundTripThroughJson() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        ByteRange range = new ByteRange(1, 2);

        String json = mapper.writeValueAsString(range);

        assertEquals("{\"start\":1,\"end\":2}", json);
        assertEquals(range, mapper.readValue(json, ByteRange.class));
    }

    @Test
    void shouldDeserializeFromJsonUsingBuilderWithUnknownFieldsIgnored() throws Exception {
        ObjectMapper mapper = new ObjectMapper();

        ByteRange range = mapper.readValue("{\"start\":5,\"end\":9,\"extra\":true}", ByteRange.class);

        assertEquals(new ByteRange(5, 9), range);
    }
}
