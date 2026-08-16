package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class PointTest {

    @Test
    void shouldExposeRowAndColumnAccessors() {
        Point point = new Point(3, 7);

        assertEquals(3, point.row());
        assertEquals(7, point.column());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        Point built = Point.builder().withRow(1).withColumn(2).build();

        assertEquals(new Point(1, 2), built);
    }

    @Test
    void shouldConsiderTwoInstancesWithSameFieldsEqual() {
        Point first = new Point(5, 9);
        Point second = new Point(5, 9);

        assertEquals(first, second);
        assertEquals(first.hashCode(), second.hashCode());
    }

    @Test
    void shouldConsiderInstancesWithDifferentFieldsUnequal() {
        Point first = new Point(5, 9);
        Point second = new Point(5, 10);

        assertNotEquals(first, second);
    }

    @Test
    void shouldRoundTripThroughJsonUsingSnakeCaseKeys() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        Point point = new Point(4, 8);

        String json = mapper.writeValueAsString(point);

        assertEquals("{\"row\":4,\"column\":8}", json);
        assertEquals(point, mapper.readValue(json, Point.class));
    }
}
