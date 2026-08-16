package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;
import org.junit.jupiter.api.Test;

class ByteArraySerializerTest {

    /** Minimal carrier so the serializer can be exercised through Jackson's normal pipeline. */
    static final class Payload {
        @JsonSerialize(using = ByteArraySerializer.class)
        public byte[] data;

        Payload(final byte[] data) {
            this.data = data;
        }
    }

    @Test
    void shouldSerializeByteArrayAsJsonArrayOfUnsignedIntegers() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        Payload payload = new Payload(new byte[] {0, 1, 127, -1, -128});

        String json = mapper.writeValueAsString(payload);

        assertEquals("{\"data\":[0,1,127,255,128]}", json);
    }

    @Test
    void shouldSerializeEmptyByteArrayAsEmptyJsonArray() throws Exception {
        ObjectMapper mapper = new ObjectMapper();
        Payload payload = new Payload(new byte[0]);

        String json = mapper.writeValueAsString(payload);

        assertEquals("{\"data\":[]}", json);
    }
}
