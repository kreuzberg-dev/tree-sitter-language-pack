package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class DocSectionTest {

    @Test
    void shouldExposeKindNameAndDescriptionAccessors() {
        DocSection section = new DocSection("Args", "count", "the number of items");

        assertEquals("Args", section.kind());
        assertEquals("count", section.name());
        assertEquals("the number of items", section.description());
    }

    @Test
    void shouldAllowNullName() {
        DocSection section = new DocSection("Returns", null, "the result");

        assertNull(section.name());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        DocSection built = DocSection.builder()
            .withKind("Raises")
            .withName("ValueError")
            .withDescription("if input is invalid")
            .build();

        assertEquals(new DocSection("Raises", "ValueError", "if input is invalid"), built);
    }

    @Test
    void shouldRoundTripThroughJson() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        DocSection section = new DocSection("Args", "x", "an integer");

        String json = mapper.writeValueAsString(section);

        assertEquals(section, mapper.readValue(json, DocSection.class));
    }
}
