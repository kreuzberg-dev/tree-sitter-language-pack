---
description: "Parsing source code with tree-sitter-language-pack — get language objects, create parsers, and navigate syntax trees."
---

# Parsing Guide

Tree-sitter syntax trees are the foundation for all downstream analysis. This guide covers parsing basics: getting language objects, configuring parsers, and navigating the resulting abstract syntax trees (ASTs).

## Quick Start

=== "Python"

    ```python
    from tree_sitter_language_pack import get_parser, parse_string

    # Get a pre-configured parser for the language
    parser = get_parser("python")
    tree = parser.parse(b"def hello():\n    print('world')\n")

    # Print the S-expression representation
    print(tree.root_node.sexp())
    ```

=== "Node.js"

    ```typescript
    import { getParser } from "@kreuzberg/tree-sitter-language-pack";

    // Get a pre-configured parser for the language
    const parser = await getParser("javascript");
    const tree = parser.parse("function hello() { console.log('world'); }");

    // Print the S-expression representation
    console.log(tree.rootNode.toString());
    ```

=== "Rust"

    ```rust
    use ts_pack_core::{get_parser, parse_string};

    // Get a pre-configured parser for the language
    let parser = get_parser("rust")?;
    let tree = parser.parse(b"fn main() {}", None)?;

    // Access the root node
    println!("{}", tree.root_node().to_sexp());
    ```

=== "CLI"

    ```bash
    # Parse a Python file and output the syntax tree
    ts-pack parse main.py

    # Or use stdin
    echo "def hello(): pass" | ts-pack parse --language python

    # Output as JSON for programmatic processing
    ts-pack parse main.py --format json
    ```

## Getting Language Objects

### `get_language(name: str) → Language`

Retrieve a tree-sitter Language object by name. This is the low-level handle that powers parsing.

=== "Python"

    ```python
    from tree_sitter_language_pack import get_language

    # Get the language object
    lang = get_language("python")

    # Use with tree-sitter Parser
    import tree_sitter
    parser = tree_sitter.Parser()
    parser.set_language(lang)
    tree = parser.parse(b"x = 1")
    ```

=== "Node.js"

    ```typescript
    import { getLanguage } from "@kreuzberg/tree-sitter-language-pack";

    // Get the language object
    const lang = await getLanguage("typescript");

    // Use with tree-sitter Parser
    const Parser = require("tree-sitter");
    const parser = new Parser();
    parser.setLanguage(lang);
    const tree = parser.parse("const x = 1;");
    ```

=== "Rust"

    ```rust
    use ts_pack_core::get_language;

    let lang = get_language("typescript")?;
    // lang is now a &Language, ready for parsing
    ```

**Language names** are case-insensitive. Common names include: `python`, `javascript`, `typescript`, `rust`, `java`, `go`, `ruby`, `php`, `c`, `cpp`, `csharp`, `kotlin`, `swift`, `elixir`, `bash`, `sql`, and [305 others](../languages.md).

### `get_parser(name: str) → Parser`

Convenience function that gets a Language and configures a tree-sitter Parser in one step.

=== "Python"

    ```python
    from tree_sitter_language_pack import get_parser

    # Get a parser ready to use immediately
    parser = get_parser("python")
    tree = parser.parse(b"x = 1")
    ```

=== "Node.js"

    ```typescript
    import { getParser } from "@kreuzberg/tree-sitter-language-pack";

    // Get a parser ready to use immediately
    const parser = await getParser("javascript");
    const tree = parser.parse("const x = 1;");
    ```

!!! tip "Auto-Download"
    By default, parsers download automatically on first use (if not cached). Set `configure(cache_dir=...)` or `$TSLP_CACHE_DIR` to use a custom cache location.

## Parsing Source Code

### `parse_string(source: str, language: str) → Tree`

Parse a string of source code directly without creating a parser object first.

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    tree = parse_string("def greet(name):\n    print(f'Hello {name}')", "python")
    print(f"Root node kind: {tree.root_node.kind}")
    print(f"Has errors: {tree.root_node.has_error}")
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    const tree = await parseString("function greet(name) { console.log(`Hello ${name}`); }", "javascript");
    console.log(`Root node kind: ${tree.rootNode.kind}`);
    console.log(`Has errors: ${tree.rootNode.hasError}`);
    ```

=== "Rust"

    ```rust
    use ts_pack_core::parse_string;

    let tree = parse_string("fn greet(name: &str) { println!(\"Hello {}\", name); }", "rust")?;
    println!("Root node kind: {}", tree.root_node().kind());
    println!("Has errors: {}", tree.root_node().has_error());
    ```

### Direct Parser.parse()

For more control, use the parser directly:

=== "Python"

    ```python
    from tree_sitter_language_pack import get_parser

    parser = get_parser("python")

    # Parse bytes (required by tree-sitter)
    source = b"def add(a, b):\n    return a + b\n"
    tree = parser.parse(source)

    # Update the same tree with new code
    new_source = b"def add(a, b, c):\n    return a + b + c\n"
    tree = parser.parse(new_source, tree)  # reuse tree for efficiency
    ```

=== "Node.js"

    ```typescript
    import { getParser } from "@kreuzberg/tree-sitter-language-pack";

    const parser = await getParser("javascript");

    let tree = parser.parse("const add = (a, b) => a + b;");
    console.log(`First parse: ${tree.rootNode.kind}`);

    // Update with new code
    const newTree = parser.parse("const add = (a, b, c) => a + b + c;");
    console.log(`Second parse: ${newTree.rootNode.kind}`);
    ```

## Understanding the Syntax Tree

### Tree Structure Basics

Tree-sitter produces a **concrete syntax tree** (CST), not an abstract syntax tree. Every token and detail appears in the tree.

```text
Example: def hello():

(module
  (function_definition
    name: (identifier)
    parameters: (parameters)
    body: (block)))
```text

Key concepts:

- **Root node**: Entry point to the tree (type: `module`, `program`, `source_file`, etc.)
- **Child nodes**: Sub-expressions, statements, declarations
- **Named nodes**: Nodes with semantic meaning (e.g., `identifier`, `call_expression`)
- **Anonymous nodes**: Tokens and operators (e.g., `(`, `)`, `def`, `:`)
- **Parent node**: The enclosing node containing the current node

### Root Node and Navigation

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    tree = parse_string("def foo(): pass", "python")
    root = tree.root_node

    print(f"Root kind: {root.kind}")                # "module"
    print(f"Start point: {root.start_point}")       # (0, 0) — line, column
    print(f"End point: {root.end_point}")           # (line, column)
    print(f"Child count: {root.child_count}")       # number of direct children
    print(f"Has error: {root.has_error}")           # True if any error nodes inside

    # Access children
    for i in range(root.child_count):
        child = root.child(i)
        print(f"Child {i}: {child.kind}")
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    const tree = await parseString("function foo() {}", "javascript");
    const root = tree.rootNode;

    console.log(`Root kind: ${root.kind}`);                    // "program"
    console.log(`Start point: (${root.startPosition.row}, ${root.startPosition.column})`);
    console.log(`End point: (${root.endPosition.row}, ${root.endPosition.column})`);
    console.log(`Child count: ${root.childCount}`);
    console.log(`Has errors: ${root.hasError}`);

    // Access children
    for (let i = 0; i < root.childCount; i++) {
      const child = root.child(i);
      console.log(`Child ${i}: ${child.kind}`);
    }
    ```

=== "Rust"

    ```rust
    use ts_pack_core::parse_string;

    let tree = parse_string("fn foo() {}", "rust")?;
    let root = tree.root_node();

    println!("Root kind: {}", root.kind());
    println!("Start: {:?}", root.start_point());
    println!("End: {:?}", root.end_point());
    println!("Child count: {}", root.child_count());
    println!("Has error: {}", root.has_error());

    // Iterate over children
    for i in 0..root.child_count() {
        if let Some(child) = root.child(i) {
            println!("Child {}: {}", i, child.kind());
        }
    }
    ```

### Navigating Child Nodes

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    tree = parse_string("""
    def greet(name):
        print(f"Hello {name}")
    """, "python")

    root = tree.root_node
    func_def = root.child(0)  # First child: function_definition

    # Navigate deeper
    name_node = func_def.child_by_field_name("name")
    params_node = func_def.child_by_field_name("parameters")
    body_node = func_def.child_by_field_name("body")

    print(f"Function name node: {name_node.text.decode()}")      # "greet"
    print(f"Parameters: {params_node.text.decode()}")            # "(name)"
    print(f"Body: {body_node.text.decode()}")                    # "print(...)"
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    const tree = await parseString(`
    function greet(name) {
      console.log(\`Hello \${name}\`);
    }
    `, "javascript");

    const root = tree.rootNode;
    const funcDecl = root.child(0);  // First child: function_declaration

    // Navigate via field names
    const nameNode = funcDecl.childByFieldName("name");
    const paramsNode = funcDecl.childByFieldName("parameters");
    const bodyNode = funcDecl.childByFieldName("body");

    console.log(`Function name: ${nameNode.text}`);             // "greet"
    console.log(`Parameters: ${paramsNode.text}`);              // "(name)"
    console.log(`Body: ${bodyNode.text}`);                      // "{ ... }"
    ```

!!! note "Field Names"
    Not all nodes have named fields. Check the grammar documentation for your language to discover available field names (e.g., `name`, `parameters`, `body`, `left`, `right`).

### Getting Node Text

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    tree = parse_string("const x = 42;", "javascript")
    root = tree.root_node

    # Get text of any node
    text = root.text.decode("utf-8")  # Tree-sitter returns bytes
    print(f"Root text: {text}")

    # For a specific child
    first_child = root.child(0)
    print(f"First child text: {first_child.text.decode()}")
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    const tree = await parseString("const x = 42;", "javascript");
    const root = tree.rootNode;

    // Get text of any node
    console.log(`Root text: ${root.text}`);

    // For a specific child
    const firstChild = root.child(0);
    console.log(`First child text: ${firstChild.text}`);
    ```

## Detecting and Handling Errors

Tree-sitter does **not** throw on invalid syntax. Instead, it marks problem areas as error nodes.

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    # Invalid Python: missing closing paren
    tree = parse_string("print('hello'", "python")

    # No exception thrown, but the tree has error markers
    print(f"Has errors: {tree.root_node.has_error}")  # True

    # Find error nodes
    def find_errors(node, errors=None):
        if errors is None:
            errors = []
        if node.kind == "ERROR":
            errors.append(node)
        for i in range(node.child_count):
            find_errors(node.child(i), errors)
        return errors

    errors = find_errors(tree.root_node)
    for err in errors:
        line, col = err.start_point
        print(f"Error at line {line}, col {col}: {err.text.decode()}")
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    // Invalid JavaScript: missing closing brace
    const tree = await parseString("function foo() {", "javascript");

    // No exception thrown, but the tree has error markers
    console.log(`Has errors: ${tree.rootNode.hasError}`);  // true

    // Find error nodes
    function findErrors(node, errors = []) {
      if (node.kind === "ERROR") {
        errors.push(node);
      }
      for (let i = 0; i < node.childCount; i++) {
        findErrors(node.child(i), errors);
      }
      return errors;
    }

    const errors = findErrors(tree.rootNode);
    errors.forEach(err => {
      const { row, column } = err.startPosition;
      console.log(`Error at line ${row}, col ${column}: ${err.text}`);
    });
    ```

### Use for Syntax Validation

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    def validate_syntax(source: str, language: str) -> tuple[bool, list[str]]:
        """Validate syntax and return (is_valid, error_messages)."""
        try:
            tree = parse_string(source, language)
        except Exception as e:
            return False, [str(e)]

        if tree.root_node.has_error:
            return False, ["Syntax errors detected"]

        return True, []

    is_valid, msgs = validate_syntax("def foo():", "python")
    if not is_valid:
        print("Syntax invalid:", msgs)
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    async function validateSyntax(source: string, language: string): Promise<[boolean, string[]]> {
      try {
        const tree = await parseString(source, language);
        if (tree.rootNode.hasError) {
          return [false, ["Syntax errors detected"]];
        }
        return [true, []];
      } catch (err) {
        return [false, [String(err)]];
      }
    }

    const [isValid, msgs] = await validateSyntax("const x = ", "javascript");
    if (!isValid) {
      console.log("Syntax invalid:", msgs);
    }
    ```

## S-Expression Format

The S-expression (symbolic expression) format is a human-readable text representation of the tree.

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    tree = parse_string("if x > 5: y = 10", "python")
    print(tree.root_node.sexp())
    ```

    **Output:**
    ```
    (module
      (if_statement
        condition: (comparison
          left: (identifier)
          operator: (comparison_operator)
          right: (integer))
        consequence: (block
          (assignment
            left: (identifier)
            right: (integer)))))
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    const tree = await parseString("if (x > 5) { y = 10; }", "javascript");
    console.log(tree.rootNode.toString());
    ```

    **Output:**
    ```
    (program
      (if_statement
        condition: (binary_expression
          left: (identifier)
          operator: ">"
          right: (number))
        consequence: (block
          (expression_statement
            (assignment_expression
              left: (identifier)
              right: (number))))))
    ```

=== "CLI"

    ```bash
    ts-pack parse example.py

    # Or as JSON
    ts-pack parse example.py --format json
    ```

## Iterating Over Nodes

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    tree = parse_string("""
    def add(a, b):
        return a + b

    def multiply(a, b):
        return a * b
    """, "python")

    def walk_tree(node):
        """Recursively visit all nodes in the tree."""
        yield node
        for i in range(node.child_count):
            child = node.child(i)
            yield from walk_tree(child)

    # Count all nodes
    node_count = sum(1 for _ in walk_tree(tree.root_node))
    print(f"Total nodes: {node_count}")

    # Find all function definitions
    func_defs = [node for node in walk_tree(tree.root_node)
                 if node.kind == "function_definition"]
    for func in func_defs:
        name = func.child_by_field_name("name").text.decode()
        print(f"Function: {name}")
    ```

=== "Node.js"

    ```typescript
    import { parseString } from "@kreuzberg/tree-sitter-language-pack";

    const tree = await parseString(`
    function add(a, b) {
      return a + b;
    }

    function multiply(a, b) {
      return a * b;
    }
    `, "javascript");

    function* walkTree(node) {
      yield node;
      for (let i = 0; i < node.childCount; i++) {
        yield* walkTree(node.child(i));
      }
    }

    // Count all nodes
    let count = 0;
    for (const _ of walkTree(tree.rootNode)) {
      count++;
    }
    console.log(`Total nodes: ${count}`);

    // Find all function declarations
    const funcDecls = [];
    for (const node of walkTree(tree.rootNode)) {
      if (node.kind === "function_declaration") {
        funcDecls.push(node);
      }
    }
    funcDecls.forEach(func => {
      const name = func.childByFieldName("name").text;
      console.log(`Function: ${name}`);
    });
    ```

## Performance Tips

!!! tip "Reuse Parsers"
    Creating a parser is relatively expensive. Reuse the same parser for multiple parses of the same language.

!!! tip "Incremental Updates"
    When re-parsing code that has changed slightly, pass the previous tree to `parser.parse()`. Tree-sitter uses the old tree to speed up parsing.

!!! tip "Lazy Navigation"
    Don't walk the entire tree if you only need specific nodes. Use field names and targeted queries instead.

!!! warning "Tree Invalidation"
    When you re-parse with `parser.parse(new_source, old_tree)`, the old tree becomes invalid. Always use the newly returned tree.

## Examples in Context

### Counting Definitions

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    source = """
    def outer():
        def inner():
            pass
        return inner

    class MyClass:
        def method(self):
            pass
    """

    tree = parse_string(source, "python")

    def count_by_kind(node, kind):
        count = 0
        def walk(n):
            nonlocal count
            if n.kind == kind:
                count += 1
            for i in range(n.child_count):
                walk(n.child(i))
        walk(node)
        return count

    funcs = count_by_kind(tree.root_node, "function_definition")
    classes = count_by_kind(tree.root_node, "class_definition")
    print(f"Functions: {funcs}, Classes: {classes}")
    # Output: Functions: 3, Classes: 1
    ```

### Finding Import Statements

=== "Python"

    ```python
    from tree_sitter_language_pack import parse_string

    source = """
    import os
    from pathlib import Path
    import sys as system
    """

    tree = parse_string(source, "python")

    imports = []
    def find_imports(node):
        if node.kind in ("import_statement", "import_from_statement"):
            imports.append(node.text.decode())
        for i in range(node.child_count):
            find_imports(node.child(i))

    find_imports(tree.root_node)
    for imp in imports:
        print(imp)
    ```

## Next Steps

- **Extract structure**: Use [`process()`](intelligence.md) to automatically extract functions, classes, imports, and more.
- **Configure parsing**: Set cache directories and pre-download languages with `configure()` and [`init()`](configuration.md).
- **Analyze code**: Explore [code intelligence](intelligence.md) to get semantically meaningful information beyond raw syntax trees.
