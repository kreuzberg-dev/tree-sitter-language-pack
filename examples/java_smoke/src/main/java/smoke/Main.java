package smoke;

import io.github.treesitter.languagepack.TsPackRegistry;

public class Main {
    public static void main(String[] args) {
        try (var registry = new TsPackRegistry()) {
            var langs = registry.availableLanguages();
            System.out.println("Available languages: " + langs.size());

            if (langs.isEmpty()) {
                throw new RuntimeException("no languages available");
            }
            if (!registry.hasLanguage("java")) {
                throw new RuntimeException("java not found");
            }

            System.out.println("Java smoke test passed");
        }
    }
}
