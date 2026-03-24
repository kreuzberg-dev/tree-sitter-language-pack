mod fixtures;
mod generators;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "e2e-generator", about = "Generate E2E test suites from JSON fixtures")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate test suites for one or all target languages.
    Generate {
        /// Target language: rust, python, typescript, go, java, elixir, c, or all.
        #[arg(long, default_value = "all")]
        lang: String,

        /// Path to the fixtures directory.
        #[arg(long, default_value = "fixtures")]
        fixtures: PathBuf,

        /// Output directory for generated tests.
        #[arg(long, default_value = "e2e")]
        output: PathBuf,
    },

    /// List all loaded fixtures.
    List {
        /// Path to the fixtures directory.
        #[arg(long, default_value = "fixtures")]
        fixtures: PathBuf,
    },

    /// Generate smoke fixtures from language_definitions.json.
    GenerateSmokeFixtures {
        /// Path to language_definitions.json.
        #[arg(long)]
        definitions: PathBuf,

        /// Output directory for generated fixture files.
        #[arg(long, default_value = "fixtures/smoke")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            lang,
            fixtures: fixtures_dir,
            output,
        } => {
            let all_fixtures = match fixtures::load_fixtures(&fixtures_dir) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error loading fixtures: {e}");
                    std::process::exit(1);
                }
            };

            eprintln!("Loaded {} fixtures from {}", all_fixtures.len(), fixtures_dir.display());

            let targets = if lang == "all" {
                generators::ALL_TARGETS.to_vec()
            } else {
                vec![lang.as_str()]
            };

            for target in &targets {
                if let Err(e) = generators::generate_for_lang(target, &all_fixtures, &output) {
                    eprintln!("Error generating {target} tests: {e}");
                    std::process::exit(1);
                }
            }

            eprintln!("Generated tests in {}", output.display());
        }

        Commands::List { fixtures: fixtures_dir } => {
            let all_fixtures = match fixtures::load_fixtures(&fixtures_dir) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error loading fixtures: {e}");
                    std::process::exit(1);
                }
            };

            println!("{:<30} {:<20} DESCRIPTION", "ID", "CATEGORY");
            println!("{}", "-".repeat(80));

            for fixture in &all_fixtures {
                println!("{:<30} {:<20} {}", fixture.id, fixture.category, fixture.description);
            }

            println!("\nTotal: {} fixtures", all_fixtures.len());
        }

        Commands::GenerateSmokeFixtures { definitions, output } => {
            let content = match std::fs::read_to_string(&definitions) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading {}: {e}", definitions.display());
                    std::process::exit(1);
                }
            };

            let defs: serde_json::Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing {}: {e}", definitions.display());
                    std::process::exit(1);
                }
            };

            let obj = match defs.as_object() {
                Some(o) => o,
                None => {
                    eprintln!("Expected top-level JSON object in {}", definitions.display());
                    std::process::exit(1);
                }
            };

            if let Err(e) = std::fs::create_dir_all(&output) {
                eprintln!("Error creating output dir: {e}");
                std::process::exit(1);
            }

            let mut count = 0;
            let source_snippets = default_source_snippets();

            for lang_name in obj.keys() {
                let source_code = source_snippets.get(lang_name.as_str()).copied().unwrap_or("x");

                let fixture = serde_json::json!({
                    "id": format!("smoke_{lang_name}"),
                    "category": "smoke",
                    "description": format!("Smoke test: load {lang_name} and parse a simple snippet"),
                    "language": lang_name,
                    "source_code": source_code,
                    "assertions": {
                        "tree_not_null": true,
                        "root_child_count_min": 1
                    },
                    "tags": ["smoke"]
                });

                let path = output.join(format!("{lang_name}.json"));
                let json = serde_json::to_string_pretty(&fixture).unwrap();

                if let Err(e) = std::fs::write(&path, format!("{json}\n")) {
                    eprintln!("Error writing {}: {e}", path.display());
                    std::process::exit(1);
                }

                count += 1;
            }

            eprintln!("Generated {count} smoke fixtures in {}", output.display());
        }
    }
}

/// Returns a map of language names to simple source code snippets.
/// Every language MUST have an entry here to avoid parser hangs or empty trees
/// from the fallback `"x"` input.
fn default_source_snippets() -> std::collections::HashMap<&'static str, &'static str> {
    let mut m = std::collections::HashMap::new();
    // Major languages
    m.insert("python", "print('hello')");
    m.insert("rust", "fn main() {}");
    m.insert("javascript", "console.log('hello');");
    m.insert("typescript", "const x: number = 42;");
    m.insert("go", "package main");
    m.insert("c", "int main() { return 0; }");
    m.insert("cpp", "int main() { return 0; }");
    m.insert("java", "class Main {}");
    m.insert("ruby", "puts 'hello'");
    m.insert("html", "<p>hello</p>");
    m.insert("css", "body { color: red; }");
    m.insert("json", "{\"key\": \"value\"}");
    m.insert("yaml", "key: value");
    m.insert("toml", "key = \"value\"");
    m.insert("bash", "echo hello");
    m.insert("lua", "print('hello')");
    m.insert("php", "<?php echo 'hello'; ?>");
    m.insert("swift", "print(\"hello\")");
    m.insert("kotlin", "fun main() {}");
    m.insert("scala", "object Main");
    m.insert("haskell", "main = putStrLn \"hello\"");
    m.insert("elixir", "IO.puts(\"hello\")");
    m.insert("erlang", "main() -> ok.");
    m.insert("ocaml", "let () = print_endline \"hello\"");
    m.insert("sql", "SELECT 1;");
    m.insert("r", "print('hello')");
    m.insert("perl", "print 'hello';");
    m.insert("zig", "pub fn main() void {}");
    // Scripting and config languages
    m.insert("actionscript", "var x:int = 1;");
    m.insert("ada", "procedure Main is begin null; end Main;");
    m.insert("agda", "module Main where");
    m.insert("apex", "public class Main {}");
    m.insert("arduino", "void setup() {}");
    m.insert("asm", "mov eax, 1");
    m.insert("astro", "---\n---\n<p>hello</p>");
    m.insert("beancount", "2024-01-01 open Assets:Bank USD");
    m.insert("bibtex", "@article{key, title={A}}");
    m.insert("bicep", "param name string");
    m.insert("bitbake", "DESCRIPTION = \"hello\"");
    m.insert("bsl", "Procedure Main() EndProcedure");
    m.insert("cairo", "fn main() {}");
    m.insert("capnp", "@0xabcdef1234567890;");
    m.insert("chatito", "%[greeting]\n    hello");
    m.insert("clarity", "(define-public (hello) (ok true))");
    m.insert("clojure", "(def x 1)");
    m.insert("cmake", "cmake_minimum_required(VERSION 3.0)");
    m.insert("cobol", "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.");
    m.insert("comment", "TODO: fix this");
    m.insert("commonlisp", "(defun hello () (print \"hello\"))");
    m.insert("cpon", "{\"key\": 1}");
    m.insert("csv", "a,b,c\n1,2,3");
    m.insert("cuda", "__global__ void kernel() {}");
    m.insert("d", "void main() {}");
    m.insert("dart", "void main() {}");
    m.insert("dockerfile", "FROM alpine");
    m.insert("doxygen", "/** @brief A function */");
    m.insert("dtd", "<!ELEMENT note (body)>");
    m.insert("elisp", "(defun hello () (message \"hello\"))");
    m.insert("elm", "module Main exposing (..)");
    m.insert("fennel", "(fn hello [] (print :hello))");
    m.insert("firrtl", "circuit Main :");
    m.insert("fish", "echo hello");
    m.insert("fortran", "program main\nend program main");
    m.insert("fsharp", "let x = 1");
    m.insert("fsharp_signature", "val x: int");
    m.insert("func", "() recv_internal() {}");
    m.insert("gdscript", "extends Node\nfunc _ready():\n\tpass");
    m.insert("gitattributes", "*.txt text");
    m.insert("gitcommit", "feat: add feature\n\nBody text");
    m.insert("gitignore", "*.o\n*.log");
    m.insert("gleam", "pub fn main() { }");
    m.insert("glsl", "void main() { gl_Position = vec4(0.0); }");
    m.insert("gn", "group(\"hello\") {}");
    m.insert("gomod", "module example.com/hello\n\ngo 1.21");
    m.insert("gosum", "example.com/pkg v1.0.0 h1:abc=");
    m.insert("graphql", "type Query { hello: String }");
    m.insert("groovy", "def x = 1");
    m.insert("gstlaunch", "fakesrc ! fakesink");
    m.insert("hack", "<?hh\nfunction main(): void {}");
    m.insert("hare", "export fn main() void = void;");
    m.insert("haxe", "class Main { static function main() {} }");
    m.insert("hcl", "variable \"name\" { type = string }");
    m.insert("heex", "<%= @greeting %>");
    m.insert("hlsl", "float4 main() : SV_Target { return 0; }");
    m.insert("hyprlang", "general { border_size = 1 }");
    m.insert("ini", "[section]\nkey = value");
    m.insert("ispc", "export void main() {}");
    m.insert("janet", "(print \"hello\")");
    m.insert("jsdoc", "/** @param {string} name */");
    m.insert("jsonnet", "{ key: 'value' }");
    m.insert("julia", "function main() end");
    m.insert("kconfig", "config FOO\n\tbool \"Enable foo\"");
    m.insert("kdl", "node \"value\"");
    m.insert(
        "latex",
        "\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}",
    );
    m.insert("linkerscript", "SECTIONS { .text : { *(.text) } }");
    m.insert("llvm", "define i32 @main() { ret i32 0 }");
    m.insert("luadoc", "---@param name string");
    m.insert("luap", "[a-z]+");
    m.insert("luau", "local x: number = 1");
    m.insert("magik", "_method object.hello\n_endmethod");
    m.insert("make", "all:\n\techo hello");
    m.insert("markdown", "# Hello\n\nWorld");
    m.insert("markdown_inline", "**bold** and *italic*");
    m.insert("matlab", "function y = hello(x)\ny = x;\nend");
    m.insert("mermaid", "graph TD\nA --> B");
    m.insert("meson", "project('hello', 'c')");
    m.insert("netlinx", "PROGRAM_NAME='hello'");
    m.insert("nim", "echo \"hello\"");
    m.insert("ninja", "rule cc\n  command = cc $in -o $out");
    m.insert("nix", "{ pkgs ? import <nixpkgs> {} }: pkgs.hello");
    m.insert("nqc", "task main() {}");
    m.insert("objc", "@interface Main @end");
    m.insert("ocaml_interface", "val x : int");
    m.insert("odin", "package main");
    m.insert("org", "* Hello\nWorld");
    m.insert("pascal", "program Hello; begin end.");
    m.insert("pem", "-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----");
    m.insert("pgn", "1. e4 e5 *");
    m.insert("po", "msgid \"hello\"\nmsgstr \"world\"");
    m.insert("pony", "actor Main\n  new create(env: Env) => None");
    m.insert("powershell", "Write-Host 'hello'");
    m.insert("printf", "%d %s");
    m.insert("prisma", "model User { id Int @id }");
    m.insert("properties", "key=value");
    m.insert("proto", "syntax = \"proto3\";");
    m.insert("psv", "a|b|c\n1|2|3");
    m.insert("puppet", "notify { 'hello': }");
    m.insert("purescript", "module Main where");
    m.insert("pymanifest", "include *.txt");
    m.insert("qmldir", "module Example");
    m.insert("qmljs", "import QtQuick 2.0\nItem {}");
    m.insert("query", "(identifier) @name");
    m.insert("racket", "#lang racket\n(define x 1)");
    m.insert("re2c", "/*!re2c\n  [a-z]+ { return; }\n*/");
    m.insert("readline", "set editing-mode vi");
    m.insert("rego", "package main\ndefault allow = false");
    m.insert("requirements", "flask>=2.0");
    m.insert("ron", "(key: \"value\")");
    m.insert("rst", "Hello\n=====\n\nWorld");
    m.insert("scheme", "(define x 1)");
    m.insert("scss", "$color: red;\nbody { color: $color; }");
    m.insert("smali", ".class public LMain;\n.super Ljava/lang/Object;");
    m.insert("smithy", "namespace example\nstring MyString");
    m.insert("solidity", "pragma solidity ^0.8.0;\ncontract Main {}");
    m.insert("sparql", "SELECT ?s WHERE { ?s ?p ?o }");
    m.insert("squirrel", "function main() {}");
    m.insert("starlark", "def hello(): pass");
    m.insert("svelte", "<script>let x = 1;</script>");
    m.insert("tablegen", "def Hello : Base {}");
    m.insert("tcl", "puts hello");
    m.insert("terraform", "resource \"null_resource\" \"main\" {}");
    m.insert("test", "===========\nTest\n===========\n---\n(node)");
    m.insert("thrift", "service HelloService {}");
    m.insert("tsv", "a\tb\tc\n1\t2\t3");
    m.insert("tsx", "const App = () => <div />;");
    m.insert("twig", "{{ variable }}");
    m.insert("typst", "#let x = 1");
    m.insert("udev", "ACTION==\"add\", KERNEL==\"sd*\"");
    m.insert("ungrammar", "Root = Item*\nItem = 'token'");
    m.insert("uxntal", "|0100 LIT 01");
    m.insert("v", "fn main() {}");
    m.insert("verilog", "module main; endmodule");
    m.insert("vhdl", "entity main is end main;");
    m.insert("vim", "echo 'hello'");
    m.insert("vue", "<template><div>hello</div></template>");
    m.insert("wast", "(module)");
    m.insert("wat", "(module)");
    m.insert(
        "wgsl",
        "@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }",
    );
    // Languages added for full coverage
    m.insert("asciidoc", "= Title\n\nParagraph.");
    m.insert("awk", "BEGIN { print \"hello\" }");
    m.insert("batch", "@echo off\necho hello");
    m.insert("caddy", ":8080 {\n\trespond \"Hello\"\n}");
    m.insert("cedar", "permit(principal, action, resource);");
    m.insert("cedarschema", "entity User;");
    m.insert("csharp", "class Main {}");
    m.insert("devicetree", "/dts-v1/;\n/ { };");
    m.insert("diff", "--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new");
    m.insert("dot", "digraph G { A -> B; }");
    m.insert("embeddedtemplate", "<%= value %>");
    m.insert("idris", "module Main");
    m.insert("jinja2", "{{ variable }}");
    m.insert("jq", ".[] | select(.key)");
    m.insert("lean", "def main : IO Unit := pure ()");
    m.insert("pkl", "name = \"hello\"");
    m.insert("postscript", "/hello { (Hello) show } def");
    m.insert("prolog", "hello :- write('hello'), nl.");
    m.insert("rescript", "let x = 1");
    m.insert("ssh_config", "Host example\n  HostName example.com");
    m.insert("textproto", "key: \"value\"");
    m.insert("tlaplus", "---- MODULE Main ----\n====");
    m.insert("vb", "Module Main\nEnd Module");
    m.insert("wit", "package example:pkg;");
    m.insert("zsh", "echo hello");
    m.insert("xcompose", "<Multi_key> <a> : \"a\"");
    m.insert("xml", "<?xml version=\"1.0\"?>\n<root>hello</root>");
    m.insert("yuck", "(defwidget main [] (label :text \"hi\"))");
    m
}
