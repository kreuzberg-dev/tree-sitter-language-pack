using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
using TreeSitterLanguagePack;
using Xunit;

namespace TestApp;

public class SmokeTest
{
    private static readonly string FixturesDir = Path.Combine(
        Directory.GetCurrentDirectory(), "..", "..", "..", "..", "fixtures");

    private static List<JsonElement> LoadFixtures(string name)
    {
        var json = File.ReadAllText(Path.Combine(FixturesDir, name));
        return JsonSerializer.Deserialize<List<JsonElement>>(json)!;
    }

    private static Dictionary<string, object?> JsonElementToDictionary(JsonElement element)
    {
        var dict = new Dictionary<string, object?>();
        if (element.ValueKind == JsonValueKind.Object)
        {
            foreach (var property in element.EnumerateObject())
            {
                dict[property.Name] = JsonElementToObject(property.Value);
            }
        }
        return dict;
    }

    private static object? JsonElementToObject(JsonElement element)
    {
        return element.ValueKind switch
        {
            JsonValueKind.String => element.GetString(),
            JsonValueKind.Number => element.GetDouble(),
            JsonValueKind.True => true,
            JsonValueKind.False => false,
            JsonValueKind.Null => null,
            JsonValueKind.Object => JsonElementToDictionary(element),
            JsonValueKind.Array => element.EnumerateArray().Select(JsonElementToObject).ToList(),
            _ => null
        };
    }

    public static void SetupLanguages()
    {
        LanguagePack.Download(new[] { "python", "javascript", "rust", "go", "ruby", "java", "c", "cpp" });
    }

    // Basic fixtures tests
    [Theory]
    [MemberData(nameof(BasicFixtures))]
    public void TestBasicFixture(Dictionary<string, object?> fixture)
    {
        var test = (string)fixture["test"]!;
        switch (test)
        {
            case "language_count":
                {
                    var count = LanguagePack.LanguageCount();
                    var expectedMin = Convert.ToInt32(fixture["expected_min"]);
                    Assert.True(count >= expectedMin,
                        $"language_count {count} < expected min {expectedMin}");
                    break;
                }
            case "has_language":
                {
                    var language = (string)fixture["language"]!;
                    var result = LanguagePack.HasLanguage(language);
                    var expected = (bool)fixture["expected"]!;
                    Assert.Equal(expected, result);
                    break;
                }
            case "available_languages":
                {
                    var langs = LanguagePack.AvailableLanguages();
                    var expectedContains = (List<object?>)fixture["expected_contains"]!;
                    foreach (var lang in expectedContains)
                    {
                        Assert.Contains((string)lang!, langs);
                    }
                    break;
                }
            default:
                throw new InvalidOperationException($"Unknown test type: {test}");
        }
    }

    public static IEnumerable<object[]> BasicFixtures()
    {
        SetupLanguages();
        var fixtures = LoadFixtures("basic.json");
        foreach (var fixture in fixtures)
        {
            var dict = JsonElementToDictionary(fixture);
            yield return new object[] { dict };
        }
    }

    // Process fixtures tests
    [Theory]
    [MemberData(nameof(ProcessFixtures))]
    public void TestProcessFixture(Dictionary<string, object?> fixture)
    {
        var source = (string)fixture["source"]!;
        var configMap = (Dictionary<string, object?>)fixture["config"]!;
        var expected = (Dictionary<string, object?>)fixture["expected"]!;

        var configJson = JsonSerializer.Serialize(configMap);
        var result = LanguagePack.Process(source, configJson);

        if (expected.ContainsKey("language"))
        {
            var resultDict = JsonElementToDictionary((JsonElement)result);
            Assert.Equal(expected["language"], resultDict["language"]);
        }

        if (expected.ContainsKey("structure_min"))
        {
            var resultDict = JsonElementToDictionary((JsonElement)result);
            var structure = (List<object?>)resultDict["structure"]!;
            var min = Convert.ToInt32(expected["structure_min"]);
            Assert.True(structure.Count >= min,
                $"structure count {structure.Count} < min {min}");
        }

        if (expected.ContainsKey("imports_min"))
        {
            var resultDict = JsonElementToDictionary((JsonElement)result);
            var imports = (List<object?>)resultDict["imports"]!;
            var min = Convert.ToInt32(expected["imports_min"]);
            Assert.True(imports.Count >= min,
                $"imports count {imports.Count} < min {min}");
        }

        if (expected.ContainsKey("error_count"))
        {
            var resultDict = JsonElementToDictionary((JsonElement)result);
            var metrics = (Dictionary<string, object?>)resultDict["metrics"]!;
            var errorCount = Convert.ToInt32(metrics["error_count"]);
            var expectedCount = Convert.ToInt32(expected["error_count"]);
            Assert.Equal(expectedCount, errorCount);
        }

        if (expected.ContainsKey("metrics_total_lines_min"))
        {
            var resultDict = JsonElementToDictionary((JsonElement)result);
            var metrics = (Dictionary<string, object?>)resultDict["metrics"]!;
            var totalLines = Convert.ToInt32(metrics["total_lines"]);
            var min = Convert.ToInt32(expected["metrics_total_lines_min"]);
            Assert.True(totalLines >= min,
                $"total_lines {totalLines} < min {min}");
        }
    }

    public static IEnumerable<object[]> ProcessFixtures()
    {
        var fixtures = LoadFixtures("process.json");
        foreach (var fixture in fixtures)
        {
            var dict = JsonElementToDictionary(fixture);
            yield return new object[] { dict };
        }
    }

    // Chunking fixtures tests
    [Theory]
    [MemberData(nameof(ChunkingFixtures))]
    public void TestChunkingFixture(Dictionary<string, object?> fixture)
    {
        var source = (string)fixture["source"]!;
        var configMap = (Dictionary<string, object?>)fixture["config"]!;
        var expected = (Dictionary<string, object?>)fixture["expected"]!;

        var configJson = JsonSerializer.Serialize(configMap);
        var result = LanguagePack.Process(source, configJson);

        if (expected.ContainsKey("chunks_min"))
        {
            var resultDict = JsonElementToDictionary((JsonElement)result);
            var chunks = (List<object?>)resultDict["chunks"]!;
            var min = Convert.ToInt32(expected["chunks_min"]);
            Assert.True(chunks.Count >= min,
                $"chunks count {chunks.Count} < min {min}");
        }
    }

    public static IEnumerable<object[]> ChunkingFixtures()
    {
        var fixtures = LoadFixtures("chunking.json");
        foreach (var fixture in fixtures)
        {
            var dict = JsonElementToDictionary(fixture);
            yield return new object[] { dict };
        }
    }

    // Download API tests
    [Fact]
    public void DownloadedLanguagesReturnsArray()
    {
        var langs = LanguagePack.DownloadedLanguages();
        Assert.NotNull(langs);
        Assert.IsAssignableFrom<List<string>>(langs);
    }

    [Fact]
    public void ManifestLanguagesReturnsArrayWith50Plus()
    {
        var langs = LanguagePack.ManifestLanguages();
        Assert.NotNull(langs);
        Assert.True(langs.Count > 50, "manifestLanguages should return 50+ languages");
    }

    [Fact]
    public void CacheDirReturnsNonEmptyString()
    {
        var dir = LanguagePack.CacheDir();
        Assert.NotNull(dir);
        Assert.True(dir.Length > 0, "cacheDir should return non-empty string");
    }

    [Fact]
    public void InitDoesNotThrow()
    {
        LanguagePack.Init();
    }

    // Parse validation tests
    [Fact]
    public void ParsesPythonCode()
    {
        var tree = LanguagePack.ParseString("python", "def hello(): pass\n");
        Assert.NotNull(tree);
        Assert.Equal("module", tree.RootNodeType());
        Assert.True(tree.RootChildCount() >= 1);
        Assert.False(tree.HasErrorNodes());
    }

    [Fact]
    public void ErrorsOnInvalidLanguage()
    {
        Assert.Throws<Exception>(() =>
            LanguagePack.ParseString("nonexistent_xyz_123", "code"));
    }

    [Fact]
    public void HasLanguageReturnsFalseForNonexistent()
    {
        var result = LanguagePack.HasLanguage("nonexistent_xyz_123");
        Assert.False(result);
    }
}
