package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNull;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class CommentInfoTest {

    private static final Span SAMPLE_SPAN = new Span(0, 5, 0, 0, 0, 5);

    @Test
    void shouldExposeAllAccessorsIncludingNullableAssociatedNode() {
        CommentInfo comment = new CommentInfo("// hello", CommentKind.Line, SAMPLE_SPAN, "foo");

        assertEquals("// hello", comment.text());
        assertEquals(CommentKind.Line, comment.kind());
        assertEquals(SAMPLE_SPAN, comment.span());
        assertEquals("foo", comment.associatedNode());
    }

    @Test
    void shouldAllowNullAssociatedNode() {
        CommentInfo comment = new CommentInfo("/* block */", CommentKind.Block, SAMPLE_SPAN, null);

        assertNull(comment.associatedNode());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        CommentInfo built = CommentInfo.builder()
            .withText("/// doc")
            .withKind(CommentKind.Doc)
            .withSpan(SAMPLE_SPAN)
            .withAssociatedNode("bar")
            .build();

        assertEquals(new CommentInfo("/// doc", CommentKind.Doc, SAMPLE_SPAN, "bar"), built);
    }

    @Test
    void shouldRoundTripThroughJsonOmittingAbsentAssociatedNode() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        CommentInfo comment = new CommentInfo("// x", CommentKind.Line, SAMPLE_SPAN, null);

        String json = mapper.writeValueAsString(comment);

        assertFalse(json.contains("associated_node"));
        assertEquals(comment, mapper.readValue(json, CommentInfo.class));
    }
}
