<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;

/**
 * Smoke test for the tree-sitter-language-pack PHP extension.
 *
 * The PHP binding is a native extension (ts_pack_php) loaded via php.ini,
 * not a Composer package. This test verifies the extension is loaded and
 * its core functions work correctly.
 */
final class SmokeTest extends TestCase
{
    protected function setUp(): void
    {
        if (!extension_loaded('ts-pack-php')) {
            $this->markTestSkipped('Extension ts_pack_php is not loaded');
        }
    }

    public function testExtensionIsLoaded(): void
    {
        $this->assertTrue(extension_loaded('ts-pack-php'), 'Extension ts_pack_php should be loaded');
    }

    public function testVersionReturnsNonEmptyString(): void
    {
        $version = \ts_pack_version();
        $this->assertIsString($version);
        $this->assertNotEmpty($version, 'ts_pack_version() should return a non-empty string');
    }

    public function testLanguageCountIsPositive(): void
    {
        $count = \ts_pack_language_count();
        $this->assertGreaterThan(0, $count, 'ts_pack_language_count() should return a positive integer');
    }

    public function testAvailableLanguagesReturnsNonEmptyArray(): void
    {
        $languages = \ts_pack_available_languages();
        $this->assertIsArray($languages);
        $this->assertNotEmpty($languages, 'ts_pack_available_languages() should return a non-empty array');
    }

    public function testHasLanguageReturnsTrueForPython(): void
    {
        $this->assertTrue(\ts_pack_has_language('python'), 'Python should be available');
    }

    public function testHasLanguageReturnsFalseForNonexistent(): void
    {
        $this->assertFalse(\ts_pack_has_language('nonexistent_xyz_123'));
    }

    public function testGetLanguageReturnsValidHandle(): void
    {
        if (!\ts_pack_has_language('python')) {
            $this->markTestSkipped('Language "python" not available');
        }
        $langPtr = \ts_pack_get_language('python');
        $this->assertIsInt($langPtr, 'Language pointer should be a valid integer handle');
        $this->assertGreaterThan(0, $langPtr);
    }

    public function testGetLanguageThrowsForUnknownLanguage(): void
    {
        $this->expectException(\Exception::class);
        \ts_pack_get_language('nonexistent_xyz_123');
    }

    public function testParseStringReturnsSExpression(): void
    {
        if (!\ts_pack_has_language('python')) {
            $this->markTestSkipped('Language "python" not available');
        }
        $sexp = \ts_pack_parse_string('python', "def hello(): pass\n");
        $this->assertIsString($sexp);
        $this->assertNotEmpty($sexp, 'Parse tree S-expression should not be empty');
        $this->assertStringContainsString('module', $sexp, 'Parse tree should contain module root node');
        $this->assertStringContainsString('function_definition', $sexp, 'Parse tree should contain function_definition node');
    }

    public function testParseStringThrowsForUnknownLanguage(): void
    {
        $this->expectException(\Exception::class);
        \ts_pack_parse_string('nonexistent_xyz_123', 'code');
    }

    public function testDetectLanguageFromPath(): void
    {
        $detected = \ts_pack_detect_language('test.py');
        $this->assertSame('python', $detected, 'Should detect Python from .py extension');
    }

    public function testDetectLanguageReturnsNullForUnknown(): void
    {
        $detected = \ts_pack_detect_language('test.unknown_ext_xyz');
        $this->assertNull($detected, 'Should return null for unknown extension');
    }
}
