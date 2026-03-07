/** Returns an array of all available language names. */
export function availableLanguages(): string[];

/** Checks whether a language with the given name is available. */
export function hasLanguage(name: string): boolean;

/** Returns the number of available languages. */
export function languageCount(): number;

/**
 * Returns the raw TSLanguage pointer as a number for interop with node-tree-sitter.
 *
 * This retrieves the compiled tree-sitter Language for the given name and returns
 * its underlying C pointer cast to a number. This can be passed to node-tree-sitter's
 * `Language` constructor.
 *
 * @throws Error if the language is not found.
 */
export function getLanguagePtr(name: string): number;
