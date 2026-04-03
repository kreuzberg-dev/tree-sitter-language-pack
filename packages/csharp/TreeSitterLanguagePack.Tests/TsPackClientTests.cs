using TreeSitterLanguagePack;
using Xunit;

namespace TreeSitterLanguagePack.Tests;

public class TsPackClientTests
{
    [Fact]
    public void AvailableLanguages_ReturnsNonEmpty()
    {
        var languages = TreeSitterLanguagePackLib.AvailableLanguages();
        Assert.NotEmpty(languages);
    }

    [Fact]
    public void LanguageCount_ReturnsPositive()
    {
        var count = TreeSitterLanguagePackLib.LanguageCount();
        Assert.True(count > 0, "should have at least one language");
    }

    [Fact]
    public void HasLanguage_ReturnsTrueForPython()
    {
        Assert.True(TreeSitterLanguagePackLib.HasLanguage("python"));
    }

    [Fact]
    public void HasLanguage_ReturnsFalseForUnknown()
    {
        Assert.False(TreeSitterLanguagePackLib.HasLanguage("nonexistent_language_xyz_42"));
    }
}
