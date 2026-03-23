//! File extension to language name mapping.
//!
//! Maps common file extensions to tree-sitter language names, enabling
//! automatic language detection from file paths.

/// Detect language name from a file extension (without leading dot).
///
/// Returns `None` for unrecognized extensions. The match is case-insensitive.
///
/// ```
/// use tree_sitter_language_pack::detect_language_from_extension;
/// assert_eq!(detect_language_from_extension("py"), Some("python"));
/// assert_eq!(detect_language_from_extension("RS"), Some("rust"));
/// assert_eq!(detect_language_from_extension("xyz"), None);
/// ```
pub fn detect_language_from_extension(ext: &str) -> Option<&'static str> {
    // Lowercase inline to avoid allocation — extensions are short.
    let mut buf = [0u8; 32];
    let ext_lower = if ext.len() <= buf.len() && ext.is_ascii() {
        for (i, b) in ext.bytes().enumerate() {
            buf[i] = b.to_ascii_lowercase();
        }
        std::str::from_utf8(&buf[..ext.len()]).ok()?
    } else {
        return None;
    };

    match ext_lower {
        // Systems / compiled
        "c" | "h" => Some("c"),
        "cpp" | "cxx" | "cc" | "hpp" | "hxx" => Some("cpp"),
        "cs" => Some("csharp"),
        "cu" | "cuda" => Some("cuda"),
        "d" => Some("d"),
        "go" => Some("go"),
        "java" => Some("java"),
        "kt" | "kts" => Some("kotlin"),
        "m" => Some("objc"),
        "rs" => Some("rust"),
        "scala" => Some("scala"),
        "swift" => Some("swift"),
        "zig" => Some("zig"),
        "v" => Some("v"),
        "odin" => Some("odin"),
        "hare" => Some("hare"),
        "nim" => Some("nim"),
        "ada" | "adb" | "ads" => Some("ada"),
        "f90" | "f95" | "f03" | "f" => Some("fortran"),
        "pas" => Some("pascal"),
        "cobol" | "cob" | "cbl" => Some("cobol"),
        "ino" => Some("arduino"),

        // Scripting / dynamic
        "py" | "pyi" => Some("python"),
        "js" | "jsx" | "mjs" | "cjs" => Some("javascript"),
        "ts" | "mts" => Some("typescript"),
        "tsx" => Some("tsx"),
        "rb" => Some("ruby"),
        "php" => Some("php"),
        "lua" => Some("lua"),
        "luau" => Some("luau"),
        "pl" | "pm" => Some("perl"),
        "r" => Some("r"),
        "jl" => Some("julia"),
        "ex" | "exs" => Some("elixir"),
        "erl" | "hrl" => Some("erlang"),
        "clj" | "cljs" | "cljc" => Some("clojure"),
        "ml" => Some("ocaml"),
        "mli" => Some("ocaml_interface"),
        "hs" => Some("haskell"),
        "fs" | "fsx" => Some("fsharp"),
        "fsi" => Some("fsharp_signature"),
        "elm" => Some("elm"),
        "purs" => Some("purescript"),
        "rkt" => Some("racket"),
        "scm" => Some("scheme"),
        "el" => Some("elisp"),
        "lisp" | "cl" => Some("commonlisp"),
        "fnl" => Some("fennel"),
        "janet" => Some("janet"),
        "dart" => Some("dart"),
        "gd" => Some("gdscript"),
        "gleam" => Some("gleam"),
        "groovy" | "gradle" => Some("groovy"),
        "tcl" => Some("tcl"),
        "fish" => Some("fish"),
        "ps1" | "psm1" | "psd1" => Some("powershell"),
        "matlab" => Some("matlab"),
        "pony" => Some("pony"),
        "hack" => Some("hack"),
        "hx" => Some("haxe"),
        "squirrel" | "nut" => Some("squirrel"),
        "nix" => Some("nix"),
        "star" | "bzl" => Some("starlark"),
        "smali" => Some("smali"),
        "pkl" => Some("pkl"),
        "vim" => Some("vim"),

        // Shell
        "sh" | "bash" => Some("bash"),
        "zsh" => Some("bash"),
        "bat" | "cmd" => Some("batch"),

        // Web / markup
        "html" | "htm" => Some("html"),
        "xml" | "xsl" | "xslt" => Some("xml"),
        "css" => Some("css"),
        "scss" => Some("scss"),
        "vue" => Some("vue"),
        "svelte" => Some("svelte"),
        "astro" => Some("astro"),
        "twig" => Some("twig"),
        "md" | "markdown" => Some("markdown"),

        // Blockchain / policy
        "sol" => Some("solidity"),
        "cairo" => Some("cairo"),
        "fc" => Some("func"),
        "clar" => Some("clarity"),
        "rego" => Some("rego"),

        // Data / config
        "json" => Some("json"),
        "jsonnet" | "libsonnet" => Some("jsonnet"),
        "toml" => Some("toml"),
        "yaml" | "yml" => Some("yaml"),
        "ini" | "cfg" => Some("ini"),
        "properties" => Some("properties"),
        "ron" => Some("ron"),
        "kdl" => Some("kdl"),
        "hcl" => Some("hcl"),
        "tf" | "tfvars" => Some("terraform"),
        "graphql" | "gql" => Some("graphql"),
        "proto" => Some("proto"),
        "thrift" => Some("thrift"),
        "capnp" => Some("capnp"),
        "smithy" => Some("smithy"),
        "prisma" => Some("prisma"),
        "beancount" => Some("beancount"),
        "sql" => Some("sql"),
        "sparql" => Some("sparql"),
        "csv" => Some("csv"),
        "tsv" => Some("tsv"),
        "psv" => Some("psv"),

        // Build / CI / docs
        "cmake" => Some("cmake"),
        "ninja" => Some("ninja"),
        "meson" => Some("meson"),
        "gn" => Some("gn"),
        "pp" => Some("puppet"),
        "tex" => Some("latex"),
        "bib" => Some("bibtex"),
        "typst" => Some("typst"),
        "dockerfile" => Some("dockerfile"),
        "bicep" => Some("bicep"),
        "mk" | "makefile" => Some("make"),
        "mod" => Some("gomod"),

        // HDL / GPU / embedded
        "vhdl" | "vhd" => Some("vhdl"),
        "sv" | "svh" | "verilog" => Some("verilog"),
        "glsl" => Some("glsl"),
        "hlsl" => Some("hlsl"),
        "wgsl" => Some("wgsl"),
        "ispc" => Some("ispc"),
        "s" | "asm" => Some("asm"),
        "ll" => Some("llvm"),
        "lds" => Some("linkerscript"),
        "wat" => Some("wat"),
        "wast" => Some("wast"),

        // Misc
        "diff" | "patch" => Some("diff"),
        "gitignore" => Some("gitignore"),
        "org" => Some("org"),
        "rst" => Some("rst"),

        _ => None,
    }
}

/// Detect language name from a file path.
///
/// Extracts the file extension and looks it up. Returns `None` if the
/// path has no extension or the extension is not recognized.
///
/// ```
/// use tree_sitter_language_pack::detect_language_from_path;
/// assert_eq!(detect_language_from_path("src/main.rs"), Some("rust"));
/// assert_eq!(detect_language_from_path("README.md"), Some("markdown"));
/// assert_eq!(detect_language_from_path("Makefile"), None);
/// ```
pub fn detect_language_from_path(path: &str) -> Option<&'static str> {
    let ext = std::path::Path::new(path).extension()?.to_str()?;
    detect_language_from_extension(ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systems_compiled() {
        assert_eq!(detect_language_from_extension("c"), Some("c"));
        assert_eq!(detect_language_from_extension("h"), Some("c"));
        assert_eq!(detect_language_from_extension("cpp"), Some("cpp"));
        assert_eq!(detect_language_from_extension("cxx"), Some("cpp"));
        assert_eq!(detect_language_from_extension("cc"), Some("cpp"));
        assert_eq!(detect_language_from_extension("hpp"), Some("cpp"));
        assert_eq!(detect_language_from_extension("hxx"), Some("cpp"));
        assert_eq!(detect_language_from_extension("cs"), Some("csharp"));
        assert_eq!(detect_language_from_extension("cu"), Some("cuda"));
        assert_eq!(detect_language_from_extension("cuda"), Some("cuda"));
        assert_eq!(detect_language_from_extension("d"), Some("d"));
        assert_eq!(detect_language_from_extension("go"), Some("go"));
        assert_eq!(detect_language_from_extension("java"), Some("java"));
        assert_eq!(detect_language_from_extension("kt"), Some("kotlin"));
        assert_eq!(detect_language_from_extension("kts"), Some("kotlin"));
        assert_eq!(detect_language_from_extension("m"), Some("objc"));
        assert_eq!(detect_language_from_extension("rs"), Some("rust"));
        assert_eq!(detect_language_from_extension("scala"), Some("scala"));
        assert_eq!(detect_language_from_extension("swift"), Some("swift"));
        assert_eq!(detect_language_from_extension("zig"), Some("zig"));
        assert_eq!(detect_language_from_extension("v"), Some("v"));
        assert_eq!(detect_language_from_extension("odin"), Some("odin"));
        assert_eq!(detect_language_from_extension("hare"), Some("hare"));
        assert_eq!(detect_language_from_extension("nim"), Some("nim"));
        assert_eq!(detect_language_from_extension("ada"), Some("ada"));
        assert_eq!(detect_language_from_extension("adb"), Some("ada"));
        assert_eq!(detect_language_from_extension("ads"), Some("ada"));
        assert_eq!(detect_language_from_extension("f90"), Some("fortran"));
        assert_eq!(detect_language_from_extension("f95"), Some("fortran"));
        assert_eq!(detect_language_from_extension("f03"), Some("fortran"));
        assert_eq!(detect_language_from_extension("f"), Some("fortran"));
        assert_eq!(detect_language_from_extension("pas"), Some("pascal"));
        assert_eq!(detect_language_from_extension("cobol"), Some("cobol"));
        assert_eq!(detect_language_from_extension("cob"), Some("cobol"));
        assert_eq!(detect_language_from_extension("cbl"), Some("cobol"));
        assert_eq!(detect_language_from_extension("ino"), Some("arduino"));
    }

    #[test]
    fn test_scripting_dynamic() {
        assert_eq!(detect_language_from_extension("py"), Some("python"));
        assert_eq!(detect_language_from_extension("pyi"), Some("python"));
        assert_eq!(detect_language_from_extension("js"), Some("javascript"));
        assert_eq!(detect_language_from_extension("jsx"), Some("javascript"));
        assert_eq!(detect_language_from_extension("mjs"), Some("javascript"));
        assert_eq!(detect_language_from_extension("cjs"), Some("javascript"));
        assert_eq!(detect_language_from_extension("ts"), Some("typescript"));
        assert_eq!(detect_language_from_extension("mts"), Some("typescript"));
        assert_eq!(detect_language_from_extension("tsx"), Some("tsx"));
        assert_eq!(detect_language_from_extension("rb"), Some("ruby"));
        assert_eq!(detect_language_from_extension("php"), Some("php"));
        assert_eq!(detect_language_from_extension("lua"), Some("lua"));
        assert_eq!(detect_language_from_extension("luau"), Some("luau"));
        assert_eq!(detect_language_from_extension("pl"), Some("perl"));
        assert_eq!(detect_language_from_extension("pm"), Some("perl"));
        assert_eq!(detect_language_from_extension("r"), Some("r"));
        assert_eq!(detect_language_from_extension("jl"), Some("julia"));
        assert_eq!(detect_language_from_extension("ex"), Some("elixir"));
        assert_eq!(detect_language_from_extension("exs"), Some("elixir"));
        assert_eq!(detect_language_from_extension("erl"), Some("erlang"));
        assert_eq!(detect_language_from_extension("hrl"), Some("erlang"));
        assert_eq!(detect_language_from_extension("clj"), Some("clojure"));
        assert_eq!(detect_language_from_extension("cljs"), Some("clojure"));
        assert_eq!(detect_language_from_extension("cljc"), Some("clojure"));
        assert_eq!(detect_language_from_extension("ml"), Some("ocaml"));
        assert_eq!(detect_language_from_extension("mli"), Some("ocaml_interface"));
        assert_eq!(detect_language_from_extension("hs"), Some("haskell"));
        assert_eq!(detect_language_from_extension("fs"), Some("fsharp"));
        assert_eq!(detect_language_from_extension("fsx"), Some("fsharp"));
        assert_eq!(detect_language_from_extension("fsi"), Some("fsharp_signature"));
        assert_eq!(detect_language_from_extension("elm"), Some("elm"));
        assert_eq!(detect_language_from_extension("purs"), Some("purescript"));
        assert_eq!(detect_language_from_extension("rkt"), Some("racket"));
        assert_eq!(detect_language_from_extension("scm"), Some("scheme"));
        assert_eq!(detect_language_from_extension("el"), Some("elisp"));
        assert_eq!(detect_language_from_extension("lisp"), Some("commonlisp"));
        assert_eq!(detect_language_from_extension("cl"), Some("commonlisp"));
        assert_eq!(detect_language_from_extension("fnl"), Some("fennel"));
        assert_eq!(detect_language_from_extension("janet"), Some("janet"));
        assert_eq!(detect_language_from_extension("dart"), Some("dart"));
        assert_eq!(detect_language_from_extension("gd"), Some("gdscript"));
        assert_eq!(detect_language_from_extension("gleam"), Some("gleam"));
        assert_eq!(detect_language_from_extension("groovy"), Some("groovy"));
        assert_eq!(detect_language_from_extension("gradle"), Some("groovy"));
        assert_eq!(detect_language_from_extension("tcl"), Some("tcl"));
        assert_eq!(detect_language_from_extension("fish"), Some("fish"));
        assert_eq!(detect_language_from_extension("ps1"), Some("powershell"));
        assert_eq!(detect_language_from_extension("psm1"), Some("powershell"));
        assert_eq!(detect_language_from_extension("psd1"), Some("powershell"));
        assert_eq!(detect_language_from_extension("matlab"), Some("matlab"));
        assert_eq!(detect_language_from_extension("pony"), Some("pony"));
        assert_eq!(detect_language_from_extension("hack"), Some("hack"));
        assert_eq!(detect_language_from_extension("hx"), Some("haxe"));
        assert_eq!(detect_language_from_extension("squirrel"), Some("squirrel"));
        assert_eq!(detect_language_from_extension("nut"), Some("squirrel"));
        assert_eq!(detect_language_from_extension("nix"), Some("nix"));
        assert_eq!(detect_language_from_extension("star"), Some("starlark"));
        assert_eq!(detect_language_from_extension("bzl"), Some("starlark"));
        assert_eq!(detect_language_from_extension("smali"), Some("smali"));
        assert_eq!(detect_language_from_extension("pkl"), Some("pkl"));
        assert_eq!(detect_language_from_extension("vim"), Some("vim"));
    }

    #[test]
    fn test_shell() {
        assert_eq!(detect_language_from_extension("sh"), Some("bash"));
        assert_eq!(detect_language_from_extension("bash"), Some("bash"));
        assert_eq!(detect_language_from_extension("zsh"), Some("bash"));
        assert_eq!(detect_language_from_extension("bat"), Some("batch"));
        assert_eq!(detect_language_from_extension("cmd"), Some("batch"));
    }

    #[test]
    fn test_web_markup() {
        assert_eq!(detect_language_from_extension("html"), Some("html"));
        assert_eq!(detect_language_from_extension("htm"), Some("html"));
        assert_eq!(detect_language_from_extension("xml"), Some("xml"));
        assert_eq!(detect_language_from_extension("xsl"), Some("xml"));
        assert_eq!(detect_language_from_extension("xslt"), Some("xml"));
        assert_eq!(detect_language_from_extension("css"), Some("css"));
        assert_eq!(detect_language_from_extension("scss"), Some("scss"));
        assert_eq!(detect_language_from_extension("vue"), Some("vue"));
        assert_eq!(detect_language_from_extension("svelte"), Some("svelte"));
        assert_eq!(detect_language_from_extension("astro"), Some("astro"));
        assert_eq!(detect_language_from_extension("twig"), Some("twig"));
        assert_eq!(detect_language_from_extension("md"), Some("markdown"));
        assert_eq!(detect_language_from_extension("markdown"), Some("markdown"));
    }

    #[test]
    fn test_blockchain_policy() {
        assert_eq!(detect_language_from_extension("sol"), Some("solidity"));
        assert_eq!(detect_language_from_extension("cairo"), Some("cairo"));
        assert_eq!(detect_language_from_extension("fc"), Some("func"));
        assert_eq!(detect_language_from_extension("clar"), Some("clarity"));
        assert_eq!(detect_language_from_extension("rego"), Some("rego"));
    }

    #[test]
    fn test_data_config() {
        assert_eq!(detect_language_from_extension("json"), Some("json"));
        assert_eq!(detect_language_from_extension("jsonnet"), Some("jsonnet"));
        assert_eq!(detect_language_from_extension("libsonnet"), Some("jsonnet"));
        assert_eq!(detect_language_from_extension("toml"), Some("toml"));
        assert_eq!(detect_language_from_extension("yaml"), Some("yaml"));
        assert_eq!(detect_language_from_extension("yml"), Some("yaml"));
        assert_eq!(detect_language_from_extension("ini"), Some("ini"));
        assert_eq!(detect_language_from_extension("cfg"), Some("ini"));
        assert_eq!(detect_language_from_extension("properties"), Some("properties"));
        assert_eq!(detect_language_from_extension("ron"), Some("ron"));
        assert_eq!(detect_language_from_extension("kdl"), Some("kdl"));
        assert_eq!(detect_language_from_extension("hcl"), Some("hcl"));
        assert_eq!(detect_language_from_extension("tf"), Some("terraform"));
        assert_eq!(detect_language_from_extension("tfvars"), Some("terraform"));
        assert_eq!(detect_language_from_extension("graphql"), Some("graphql"));
        assert_eq!(detect_language_from_extension("gql"), Some("graphql"));
        assert_eq!(detect_language_from_extension("proto"), Some("proto"));
        assert_eq!(detect_language_from_extension("thrift"), Some("thrift"));
        assert_eq!(detect_language_from_extension("capnp"), Some("capnp"));
        assert_eq!(detect_language_from_extension("smithy"), Some("smithy"));
        assert_eq!(detect_language_from_extension("prisma"), Some("prisma"));
        assert_eq!(detect_language_from_extension("beancount"), Some("beancount"));
        assert_eq!(detect_language_from_extension("sql"), Some("sql"));
        assert_eq!(detect_language_from_extension("sparql"), Some("sparql"));
        assert_eq!(detect_language_from_extension("csv"), Some("csv"));
        assert_eq!(detect_language_from_extension("tsv"), Some("tsv"));
        assert_eq!(detect_language_from_extension("psv"), Some("psv"));
    }

    #[test]
    fn test_build_ci_docs() {
        assert_eq!(detect_language_from_extension("cmake"), Some("cmake"));
        assert_eq!(detect_language_from_extension("ninja"), Some("ninja"));
        assert_eq!(detect_language_from_extension("meson"), Some("meson"));
        assert_eq!(detect_language_from_extension("gn"), Some("gn"));
        assert_eq!(detect_language_from_extension("pp"), Some("puppet"));
        assert_eq!(detect_language_from_extension("tex"), Some("latex"));
        assert_eq!(detect_language_from_extension("bib"), Some("bibtex"));
        assert_eq!(detect_language_from_extension("typst"), Some("typst"));
        assert_eq!(detect_language_from_extension("dockerfile"), Some("dockerfile"));
        assert_eq!(detect_language_from_extension("bicep"), Some("bicep"));
        assert_eq!(detect_language_from_extension("mk"), Some("make"));
        assert_eq!(detect_language_from_extension("makefile"), Some("make"));
        assert_eq!(detect_language_from_extension("mod"), Some("gomod"));
    }

    #[test]
    fn test_hdl_gpu_embedded() {
        assert_eq!(detect_language_from_extension("vhdl"), Some("vhdl"));
        assert_eq!(detect_language_from_extension("vhd"), Some("vhdl"));
        assert_eq!(detect_language_from_extension("sv"), Some("verilog"));
        assert_eq!(detect_language_from_extension("svh"), Some("verilog"));
        assert_eq!(detect_language_from_extension("verilog"), Some("verilog"));
        assert_eq!(detect_language_from_extension("glsl"), Some("glsl"));
        assert_eq!(detect_language_from_extension("hlsl"), Some("hlsl"));
        assert_eq!(detect_language_from_extension("wgsl"), Some("wgsl"));
        assert_eq!(detect_language_from_extension("ispc"), Some("ispc"));
        assert_eq!(detect_language_from_extension("s"), Some("asm"));
        assert_eq!(detect_language_from_extension("asm"), Some("asm"));
        assert_eq!(detect_language_from_extension("ll"), Some("llvm"));
        assert_eq!(detect_language_from_extension("lds"), Some("linkerscript"));
        assert_eq!(detect_language_from_extension("wat"), Some("wat"));
        assert_eq!(detect_language_from_extension("wast"), Some("wast"));
    }

    #[test]
    fn test_misc() {
        assert_eq!(detect_language_from_extension("diff"), Some("diff"));
        assert_eq!(detect_language_from_extension("patch"), Some("diff"));
        assert_eq!(detect_language_from_extension("gitignore"), Some("gitignore"));
        assert_eq!(detect_language_from_extension("org"), Some("org"));
        assert_eq!(detect_language_from_extension("rst"), Some("rst"));
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(detect_language_from_extension("PY"), Some("python"));
        assert_eq!(detect_language_from_extension("Rs"), Some("rust"));
        assert_eq!(detect_language_from_extension("JS"), Some("javascript"));
        assert_eq!(detect_language_from_extension("CPP"), Some("cpp"));
        assert_eq!(detect_language_from_extension("Tsx"), Some("tsx"));
    }

    #[test]
    fn test_unknown() {
        assert_eq!(detect_language_from_extension("xyz"), None);
        assert_eq!(detect_language_from_extension(""), None);
        assert_eq!(detect_language_from_extension("abcdef"), None);
    }

    #[test]
    fn test_path_detection() {
        assert_eq!(detect_language_from_path("src/main.rs"), Some("rust"));
        assert_eq!(detect_language_from_path("/path/to/file.py"), Some("python"));
        assert_eq!(detect_language_from_path("README.md"), Some("markdown"));
        assert_eq!(detect_language_from_path("app.test.tsx"), Some("tsx"));
        assert_eq!(detect_language_from_path("Cargo.toml"), Some("toml"));
    }

    #[test]
    fn test_path_no_extension() {
        assert_eq!(detect_language_from_path("Makefile"), None);
        assert_eq!(detect_language_from_path(""), None);
        assert_eq!(detect_language_from_path("/usr/bin/env"), None);
    }

    #[test]
    fn test_long_extension_rejected() {
        // Extensions longer than 32 bytes return None (no allocation)
        let long = "a".repeat(33);
        assert_eq!(detect_language_from_extension(&long), None);
    }
}
