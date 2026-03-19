import { describe, expect, it, beforeAll } from "vitest";
import { process, download } from "@kreuzberg/tree-sitter-language-pack";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

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


describe("process tests", () => {
	beforeAll(() => {
		download(["python", "javascript", "rust", "go"]);
	});

	const fixtures = loadFixtures<ProcessFixture>("process.json");

	for (const fixture of fixtures) {
		it(fixture.name, () => {
			const result = process(fixture.source, fixture.config);
			const expected = fixture.expected;

			if ("language" in expected) {
				expect(result.language).toBe(expected.language);
			}
			if ("structure_min" in expected) {
				expect(result.structure.length).toBeGreaterThanOrEqual(
					expected.structure_min as number,
				);
			}
			if ("imports_min" in expected) {
				expect(result.imports.length).toBeGreaterThanOrEqual(
					expected.imports_min as number,
				);
			}
			if ("error_count" in expected) {
				expect(result.metrics.errorCount).toBe(expected.error_count);
			}
		});
	}
});

describe("chunking tests", () => {
	const fixtures = loadFixtures<ProcessFixture>("chunking.json");

	for (const fixture of fixtures) {
		it(fixture.name, () => {
			const result = process(fixture.source, fixture.config);
			const expected = fixture.expected;

			if ("chunks_min" in expected) {
				expect(result.chunks.length).toBeGreaterThanOrEqual(
					expected.chunks_min as number,
				);
			}
		});
	}
});
