import { describe, expect, it, beforeAll } from "vitest";
import {
	availableLanguages,
	hasLanguage,
	languageCount,
	parseString,
	treeRootNodeType,
	treeRootChildCount,
	treeHasErrorNodes,
	init,
	downloadedLanguages,
	manifestLanguages,
	cacheDir,
} from "@kreuzberg/tree-sitter-language-pack";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

interface BasicFixture {
	name: string;
	test: string;
	language?: string;
	expected?: boolean;
	expected_min?: number;
	expected_contains?: string[];
}

const fixturesDir = resolve(import.meta.dirname, "..", "fixtures");

function loadFixtures<T>(name: string): T[] {
	return JSON.parse(readFileSync(resolve(fixturesDir, name), "utf-8"));
}

import { download } from "@kreuzberg/tree-sitter-language-pack";

describe("smoke tests", () => {
	beforeAll(() => {
		download(["python", "javascript", "rust", "go", "ruby", "java", "c", "cpp"]);
	});

	describe("basic fixtures", () => {
		const fixtures = loadFixtures<BasicFixture>("basic.json");

		for (const fixture of fixtures) {
			it(fixture.name, () => {
				switch (fixture.test) {
					case "language_count": {
						const count = languageCount();
						expect(count).toBeGreaterThanOrEqual(fixture.expected_min!);
						break;
					}
					case "has_language": {
						const result = hasLanguage(fixture.language!);
						expect(result).toBe(fixture.expected);
						break;
					}
					case "available_languages": {
						const langs = availableLanguages();
						for (const lang of fixture.expected_contains!) {
							expect(langs).toContain(lang);
						}
						break;
					}
					default:
						throw new Error(`Unknown test type: ${fixture.test}`);
				}
			});
		}
	});

	describe("parse validation", () => {
		it("parses Python code", () => {
			const tree = parseString("python", "def hello(): pass\n");
			expect(treeRootNodeType(tree)).toBe("module");
			expect(treeRootChildCount(tree)).toBeGreaterThanOrEqual(1);
			expect(treeHasErrorNodes(tree)).toBe(false);
		});

		it("throws on invalid language", () => {
			expect(() => parseString("nonexistent_xyz_123", "code")).toThrow();
		});
	});

	describe("download API", () => {

		it("exposes all download functions", async () => {
			const functions = [
				"init",
				"download",
				"downloadAll",
				"configure",
				"manifestLanguages",
				"downloadedLanguages",
				"cleanCache",
				"cacheDir",
			];

			// Use dynamic import to load bindings in ESM context
			const tslp = await import("@kreuzberg/tree-sitter-language-pack");
			for (const fn of functions) {
				expect(typeof tslp[fn as keyof typeof tslp]).toBe("function");
			}
		});

		it("downloadedLanguages returns array", () => {
			const langs = downloadedLanguages();
			expect(Array.isArray(langs)).toBe(true);
		});

		it("manifestLanguages returns array with 50+ languages", () => {
			const langs = manifestLanguages();
			expect(Array.isArray(langs)).toBe(true);
			expect(langs.length).toBeGreaterThan(50);
		});

		it("cacheDir returns non-empty string", () => {
			const dir = cacheDir();
			expect(typeof dir).toBe("string");
			expect(dir.length).toBeGreaterThan(0);
		});
	});
});
