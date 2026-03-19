import { describe, expect, it, beforeAll } from "vitest";
import init, {
	availableLanguages,
	hasLanguage,
	languageCount,
	parseString,
	treeRootNodeType,
	treeHasErrorNodes,
	treeRootChildCount,
	treeContainsNodeType,
	process,
} from "@kreuzberg/tree-sitter-language-pack-wasm";
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

interface ProcessFixture {
	name: string;
	test: string;
	source: string;
	config: Record<string, unknown>;
	expected: Record<string, unknown>;
}

const fixturesDir = resolve(import.meta.dirname, "..", "fixtures");

function loadFixtures<T>(name: string): T[] {
	return JSON.parse(readFileSync(resolve(fixturesDir, name), "utf-8"));
}

describe("wasm smoke tests", () => {
	beforeAll(async () => {
		await init();
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
			expect(treeHasErrorNodes(tree)).toBe(false);
		});

		it("throws on invalid language", () => {
			expect(() => parseString("nonexistent_xyz_123", "code")).toThrow();
		});

		it("parses JavaScript code", () => {
			const tree = parseString("javascript", "function test() { return 1; }\n");
			expect(treeRootNodeType(tree)).toBe("program");
			expect(treeRootChildCount(tree)).toBeGreaterThanOrEqual(1);
			expect(treeHasErrorNodes(tree)).toBe(false);
		});

		it("parses Rust code", () => {
			const tree = parseString("rust", "fn main() { println!(\"hello\"); }\n");
			expect(treeRootNodeType(tree)).toBe("source_file");
			expect(treeHasErrorNodes(tree)).toBe(false);
		});

		it("contains function_definition in Python", () => {
			const tree = parseString("python", "def test():\n    pass\n");
			expect(treeContainsNodeType(tree, "function_definition")).toBe(true);
		});

		it("returns error count for invalid syntax", () => {
			const tree = parseString("python", "def broken(:\n");
			expect(treeHasErrorNodes(tree)).toBe(true);
		});
	});

	describe("process API tests", () => {
		const fixtures = loadFixtures<ProcessFixture>("process.json");

		for (const fixture of fixtures) {
			it(fixture.name, () => {
				const result = process(fixture.source, fixture.config);

				// Check if result is a valid object
				expect(result).toBeTruthy();
				expect(typeof result).toBe("object");

				// Verify language matches
				if (fixture.expected.language) {
					expect(result.language).toBe(fixture.expected.language);
				}

				// Verify structure count
				if (typeof fixture.expected.structure_min === "number") {
					expect((result.structure || []).length).toBeGreaterThanOrEqual(
						fixture.expected.structure_min
					);
				}

				// Verify error count
				if (typeof fixture.expected.error_count === "number") {
					expect(result.error_count || 0).toBe(fixture.expected.error_count);
				}

				// Verify metrics if present
				if (fixture.expected.metrics_total_lines_min) {
					expect(result.metrics?.total_lines || 0).toBeGreaterThanOrEqual(
						fixture.expected.metrics_total_lines_min as number
					);
				}

				// Verify imports if expected
				if (typeof fixture.expected.imports_min === "number") {
					expect((result.imports || []).length).toBeGreaterThanOrEqual(
						fixture.expected.imports_min
					);
				}
			});
		}
	});

	describe("chunking tests", () => {
		const fixtures = loadFixtures<ProcessFixture>("chunking.json");

		for (const fixture of fixtures) {
			it(fixture.name, () => {
				const result = process(fixture.source, fixture.config);

				// Check if result is a valid object
				expect(result).toBeTruthy();
				expect(typeof result).toBe("object");

				// Verify chunks exist and meet minimum count
				if (typeof fixture.expected.chunks_min === "number") {
					expect((result.chunks || []).length).toBeGreaterThanOrEqual(
						fixture.expected.chunks_min
					);
				}
			});
		}
	});
});
