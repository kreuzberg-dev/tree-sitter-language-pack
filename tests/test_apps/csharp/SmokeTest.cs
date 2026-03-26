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
                    var count = TsPackClient.LanguageCount();
                    var expectedMin = Convert.ToInt32(fixture["expected_min"]);
                    Assert.True(count >= expectedMin,
                        $"language_count {count} < expected min {expectedMin}");
                    break;
                }
            case "has_language":
                {
                    var language = (string)fixture["language"]!;
                    var result = TsPackClient.HasLanguage(language);
                    var expected = (bool)fixture["expected"]!;
                    Assert.Equal(expected, result);
                    break;
                }
            case "available_languages":
                {
                    var langs = TsPackClient.AvailableLanguages();
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

        var config = new ProcessConfig
        {
            Language = (string)configMap["language"]!
        };

        if (configMap.TryGetValue("structure", out var structureVal))
        {
            config.Structure = (bool)structureVal!;
        }
        if (configMap.TryGetValue("imports", out var importsVal))
        {
            config.Imports = (bool)importsVal!;
        }
        if (configMap.TryGetValue("exports", out var exportsVal))
        {
            config.Exports = (bool)exportsVal!;
        }
        if (configMap.TryGetValue("comments", out var commentsVal))
        {
            config.Comments = (bool)commentsVal!;
        }
        if (configMap.TryGetValue("chunk_max_size", out var chunkVal))
        {
            config.ChunkMaxSize = Convert.ToInt32(chunkVal);
        }

        var expected = (Dictionary<string, object?>)fixture["expected"]!;
        var result = TsPackClient.Process(source, config);

        if (expected.TryGetValue("language", out var expectedLang))
        {
            Assert.Equal((string)expectedLang!, result.Language);
        }

        if (expected.TryGetValue("structure_min", out var structureMin))
        {
            var min = Convert.ToInt32(structureMin);
            Assert.True(result.Structure.Count >= min,
                $"structure count {result.Structure.Count} < min {min}");
        }

        if (expected.TryGetValue("imports_min", out var importsMin))
        {
            var min = Convert.ToInt32(importsMin);
            Assert.True(result.Imports.Count >= min,
                $"imports count {result.Imports.Count} < min {min}");
        }

        if (expected.TryGetValue("error_count", out var errorCount))
        {
            var expectedCount = Convert.ToInt32(errorCount);
            Assert.Equal(expectedCount, result.Metrics.ErrorCount);
        }

        if (expected.TryGetValue("metrics_total_lines_min", out var totalLinesMin))
        {
            var min = Convert.ToInt32(totalLinesMin);
            Assert.True(result.Metrics.TotalLines >= min,
                $"total_lines {result.Metrics.TotalLines} < min {min}");
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

        var config = new ProcessConfig
        {
            Language = (string)configMap["language"]!
        };

        if (configMap.TryGetValue("structure", out var structureVal))
        {
            config.Structure = (bool)structureVal!;
        }
        if (configMap.TryGetValue("chunk_max_size", out var chunkVal))
        {
            config.ChunkMaxSize = Convert.ToInt32(chunkVal);
        }

        var result = TsPackClient.Process(source, config);

        if (expected.TryGetValue("chunks_min", out var chunksMin))
        {
            var min = Convert.ToInt32(chunksMin);
            Assert.True(result.Chunks.Count >= min,
                $"chunks count {result.Chunks.Count} < min {min}");
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

    // Parse validation tests
    [Fact]
    public void ParsesPythonCode()
    {
        using var tree = TsPackClient.Parse("python", "def hello(): pass\n");
        Assert.NotNull(tree);
        Assert.Equal("module", tree.RootNodeType());
        Assert.True(tree.RootChildCount() >= 1);
        Assert.False(tree.HasErrorNodes());
    }

    [Fact]
    public void ErrorsOnInvalidLanguage()
    {
        Assert.Throws<TsPackException>(() =>
            TsPackClient.Parse("nonexistent_xyz_123", "code"));
    }

    [Fact]
    public void HasLanguageReturnsFalseForNonexistent()
    {
        var result = TsPackClient.HasLanguage("nonexistent_xyz_123");
        Assert.False(result);
    }

    [Fact]
    public void AvailableLanguagesReturnsNonEmptyArray()
    {
        var langs = TsPackClient.AvailableLanguages();
        Assert.NotNull(langs);
        Assert.True(langs.Length > 0, "AvailableLanguages should return at least one language");
        Assert.Contains("python", langs);
        Assert.Contains("javascript", langs);
    }

    [Fact]
    public void LanguageCountIsPositive()
    {
        var count = TsPackClient.LanguageCount();
        Assert.True(count > 0, "LanguageCount should be positive");
    }

    [Fact]
    public void ParseTreeDisposesCleanly()
    {
        var tree = TsPackClient.Parse("python", "x = 1\n");
        tree.Dispose();
        // Second dispose should be a safe no-op
        tree.Dispose();
    }

    [Fact]
    public void ParseTreeHasErrorNodesForInvalidSyntax()
    {
        using var tree = TsPackClient.Parse("python", "def ():\n");
        Assert.True(tree.HasErrorNodes(), "Invalid Python syntax should produce error nodes");
        Assert.True(tree.ErrorCount() > 0, "ErrorCount should be positive for invalid syntax");
    }

    [Fact]
    public void ParseTreeContainsNodeType()
    {
        using var tree = TsPackClient.Parse("python", "def hello(): pass\n");
        Assert.True(tree.ContainsNodeType("function_definition"),
            "Python function should contain function_definition node");
    }

    [Fact]
    public void ParseTreeToSexpReturnsNonEmpty()
    {
        using var tree = TsPackClient.Parse("python", "x = 1\n");
        var sexp = tree.ToSexp();
        Assert.NotNull(sexp);
        Assert.True(sexp!.Length > 0, "S-expression should be non-empty");
    }
}
