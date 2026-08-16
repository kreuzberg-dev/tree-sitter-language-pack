package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.List;
import org.junit.jupiter.api.Test;

class CodeChunkTest {

    private static final ChunkContext SAMPLE_CONTEXT = new ChunkContext(
        "python", 0, 1, List.of(), List.of(), List.of(), List.of(), List.of(), false
    );

    @Test
    void shouldExposeAllAccessors() {
        CodeChunk chunk = new CodeChunk("def f(): pass", 0, 14, 1, 1, SAMPLE_CONTEXT);

        assertEquals("def f(): pass", chunk.content());
        assertEquals(0, chunk.startByte());
        assertEquals(14, chunk.endByte());
        assertEquals(1, chunk.startLine());
        assertEquals(1, chunk.endLine());
        assertEquals(SAMPLE_CONTEXT, chunk.metadata());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        CodeChunk built = CodeChunk.builder()
            .withContent("x = 1")
            .withStartByte(0)
            .withEndByte(5)
            .withStartLine(1)
            .withEndLine(1)
            .withMetadata(SAMPLE_CONTEXT)
            .build();

        assertEquals(new CodeChunk("x = 1", 0, 5, 1, 1, SAMPLE_CONTEXT), built);
    }

    @Test
    void shouldRoundTripThroughJsonWithNestedMetadata() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        CodeChunk chunk = new CodeChunk("y = 2", 0, 5, 1, 1, SAMPLE_CONTEXT);

        String json = mapper.writeValueAsString(chunk);
        CodeChunk parsed = mapper.readValue(json, CodeChunk.class);

        assertEquals(chunk, parsed);
        assertEquals("python", parsed.metadata().language());
    }
}
