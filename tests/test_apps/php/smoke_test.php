<?php

declare(strict_types=1);

use PHPUnit\Framework\TestCase;
use TreeSitterLanguagePack\LanguagePack;

final class SmokeTest extends TestCase
{
    private static string $fixturesDir;
    private LanguagePack $pack;

    public static function setUpBeforeClass(): void
    {
        self::$fixturesDir = dirname(__DIR__) . '/fixtures';
        // Download required languages
        LanguagePack::download(['python', 'javascript', 'rust', 'go', 'ruby', 'java', 'c', 'cpp']);
    }

    protected function setUp(): void
    {
        $this->pack = new LanguagePack();
    }

    private static function loadFixtures(string $name): array
    {
        $json = file_get_contents(self::$fixturesDir . '/' . $name);
        return json_decode($json, true, 512, JSON_THROW_ON_ERROR);
    }

    /**
     * @dataProvider basicFixtureProvider
     */
    public function testBasicFixture(array $fixture): void
    {
        match ($fixture['test']) {
            'language_count' => $this->assertGreaterThanOrEqual(
                $fixture['expected_min'],
                $this->pack->languageCount(),
                "language_count < expected min {$fixture['expected_min']}"
            ),
            'has_language' => $this->assertSame(
                $fixture['expected'],
                $this->pack->hasLanguage($fixture['language']),
                "has_language({$fixture['language']})"
            ),
            'available_languages' => $this->assertAvailableLanguagesContains(
                $fixture['expected_contains']
            ),
            default => $this->fail("Unknown test type: {$fixture['test']}"),
        };
    }

    public static function basicFixtureProvider(): iterable
    {
        $fixtures = self::loadFixtures('basic.json');
        foreach ($fixtures as $fixture) {
            yield $fixture['name'] => [$fixture];
        }
    }

    /**
     * @dataProvider processFixtureProvider
     */
    public function testProcessFixture(array $fixture): void
    {
        $source = $fixture['source'];
        $config = json_encode($fixture['config']);
        $result = $this->pack->process($source, $config);
        $expected = $fixture['expected'];

        if (isset($expected['language'])) {
            $this->assertSame($expected['language'], $result['language']);
        }
        if (isset($expected['structure_min'])) {
            $structureCount = count($result['structure'] ?? []);
            $this->assertGreaterThanOrEqual($expected['structure_min'], $structureCount,
                "structure count {$structureCount} < min {$expected['structure_min']}");
        }
        if (isset($expected['imports_min'])) {
            $importsCount = count($result['imports'] ?? []);
            $this->assertGreaterThanOrEqual($expected['imports_min'], $importsCount,
                "imports count {$importsCount} < min {$expected['imports_min']}");
        }
        if (isset($expected['error_count'])) {
            $errorCount = $result['metrics']['error_count'] ?? 0;
            $this->assertSame($expected['error_count'], $errorCount);
        }
        if (isset($expected['metrics_total_lines_min'])) {
            $totalLines = $result['metrics']['total_lines'] ?? 0;
            $this->assertGreaterThanOrEqual($expected['metrics_total_lines_min'], $totalLines,
                "total_lines {$totalLines} < min {$expected['metrics_total_lines_min']}");
        }
    }

    public static function processFixtureProvider(): iterable
    {
        $fixtures = self::loadFixtures('process.json');
        foreach ($fixtures as $fixture) {
            yield $fixture['name'] => [$fixture];
        }
    }

    /**
     * @dataProvider chunkingFixtureProvider
     */
    public function testChunkingFixture(array $fixture): void
    {
        $source = $fixture['source'];
        $config = json_encode($fixture['config']);
        $result = $this->pack->process($source, $config);
        $expected = $fixture['expected'];

        if (isset($expected['chunks_min'])) {
            $chunksCount = count($result['chunks'] ?? []);
            $this->assertGreaterThanOrEqual($expected['chunks_min'], $chunksCount,
                "chunks count {$chunksCount} < min {$expected['chunks_min']}");
        }
    }

    public static function chunkingFixtureProvider(): iterable
    {
        $fixtures = self::loadFixtures('chunking.json');
        foreach ($fixtures as $fixture) {
            yield $fixture['name'] => [$fixture];
        }
    }

    private function assertAvailableLanguagesContains(array $expected): void
    {
        $langs = $this->pack->availableLanguages();
        foreach ($expected as $lang) {
            $this->assertContains($lang, $langs, "available_languages missing '{$lang}'");
        }
    }

    // Download API tests
    public function testDownloadedLanguagesReturnsArray(): void
    {
        $langs = LanguagePack::downloadedLanguages();
        $this->assertIsArray($langs);
    }

    public function testManifestLanguagesReturnsArrayWith50Plus(): void
    {
        $langs = LanguagePack::manifestLanguages();
        $this->assertIsArray($langs);
        $this->assertGreaterThan(50, count($langs), "manifestLanguages should return 50+ languages");
    }

    public function testCacheDirReturnsNonEmptyString(): void
    {
        $dir = LanguagePack::cacheDir();
        $this->assertIsString($dir);
        $this->assertGreaterThan(0, strlen($dir), "cacheDir should return non-empty string");
    }

    public function testInitDoesNotThrow(): void
    {
        LanguagePack::init();
        $this->assertTrue(true);
    }

    // Parse validation tests
    public function testParsesPythonCode(): void
    {
        $tree = $this->pack->parseString('python', "def hello(): pass\n");
        $this->assertSame('module', $tree->rootNodeType());
        $this->assertGreaterThanOrEqual(1, $tree->rootChildCount());
        $this->assertFalse($tree->hasErrorNodes());
    }

    public function testErrorsOnInvalidLanguage(): void
    {
        $this->expectException(\RuntimeException::class);
        $this->pack->parseString('nonexistent_xyz_123', 'code');
    }

    public function testHasLanguageReturnsFalseForNonexistent(): void
    {
        $result = $this->pack->hasLanguage('nonexistent_xyz_123');
        $this->assertFalse($result);
    }
}
