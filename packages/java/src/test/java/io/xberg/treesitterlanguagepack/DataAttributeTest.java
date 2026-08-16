package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class DataAttributeTest {

    private static final Span SAMPLE_SPAN = new Span(0, 3, 0, 0, 0, 3);

    @Test
    void shouldExposeNameValueAndSpanAccessors() {
        DataAttribute attribute = new DataAttribute("id", "42", SAMPLE_SPAN);

        assertEquals("id", attribute.name());
        assertEquals("42", attribute.value());
        assertEquals(SAMPLE_SPAN, attribute.span());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        DataAttribute built = DataAttribute.builder().withName("class").withValue("foo").withSpan(SAMPLE_SPAN).build();

        assertEquals(new DataAttribute("class", "foo", SAMPLE_SPAN), built);
    }

    @Test
    void shouldRoundTripThroughJson() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        DataAttribute attribute = new DataAttribute("href", "https://example.com", SAMPLE_SPAN);

        String json = mapper.writeValueAsString(attribute);

        assertEquals(attribute, mapper.readValue(json, DataAttribute.class));
    }
}
