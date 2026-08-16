package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNull;

import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import org.junit.jupiter.api.Test;

class PackConfigTest {

    @Test
    void shouldExposeAllAccessors() {
        Path cacheDir = Paths.get("/tmp/tslp-cache");
        PackConfig config = new PackConfig(cacheDir, List.of("python", "rust"), List.of("web"));

        assertEquals(cacheDir, config.cacheDir());
        assertEquals(List.of("python", "rust"), config.languages());
        assertEquals(List.of("web"), config.groups());
    }

    @Test
    void shouldAllowAllFieldsToBeNull() {
        PackConfig config = new PackConfig(null, null, null);

        assertNull(config.cacheDir());
        assertNull(config.languages());
        assertNull(config.groups());
    }

    @Test
    void shouldBuildEquivalentInstanceThroughBuilder() {
        Path cacheDir = Paths.get("cache");
        PackConfig built = PackConfig.builder()
            .withCacheDir(cacheDir)
            .withLanguages(List.of("go"))
            .withGroups(null)
            .build();

        assertEquals(new PackConfig(cacheDir, List.of("go"), null), built);
    }
}
