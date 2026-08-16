package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class FileMetricsTest {

    @Test
    void shouldExposeAllEightAccessors() {
        FileMetrics metrics = new FileMetrics(100, 70, 20, 10, 2048, 500, 3, 12);

        assertEquals(100, metrics.totalLines());
        assertEquals(70, metrics.codeLines());
        assertEquals(20, metrics.commentLines());
        assertEquals(10, metrics.blankLines());
        assertEquals(2048, metrics.totalBytes());
        assertEquals(500, metrics.nodeCount());
        assertEquals(3, metrics.errorCount());
        assertEquals(12, metrics.maxDepth());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        FileMetrics built = FileMetrics.builder()
            .withTotalLines(10)
            .withCodeLines(8)
            .withCommentLines(1)
            .withBlankLines(1)
            .withTotalBytes(256)
            .withNodeCount(40)
            .withErrorCount(0)
            .withMaxDepth(5)
            .build();

        assertEquals(new FileMetrics(10, 8, 1, 1, 256, 40, 0, 5), built);
    }

    @Test
    void shouldRoundTripThroughJsonUsingSnakeCaseKeys() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        FileMetrics metrics = new FileMetrics(1, 1, 0, 0, 10, 1, 0, 1);

        String json = mapper.writeValueAsString(metrics);

        assertTrue(json.contains("\"error_count\":0"));
        assertEquals(metrics, mapper.readValue(json, FileMetrics.class));
    }
}
