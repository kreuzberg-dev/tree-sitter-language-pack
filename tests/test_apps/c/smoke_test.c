#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include "ts_pack.h"

#define ASSERT(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "FAIL: %s\n", msg); \
        failures++; \
    } else { \
        printf("  PASS: %s\n", msg); \
    } \
} while (0)

int main(void) {
    int failures = 0;

    printf("=== C Smoke Tests ===\n\n");

    /* ====== BASIC TESTS (9) ====== */
    printf("-- Basic Tests --\n");

    /* Initialize the language pack */
    int32_t init_result = ts_pack_init(NULL);
    ASSERT(init_result == 0, "init() succeeds");

    /* Create registry */
    TsPackRegistry *registry = ts_pack_registry_new();
    ASSERT(registry != NULL, "registry creation");

    /* Language count */
    uintptr_t count = ts_pack_language_count(registry);
    ASSERT(count >= 100, "language_count >= 100");

    /* Has language checks */
    ASSERT(ts_pack_has_language(registry, "python") == true, "has_language(python)");
    ASSERT(ts_pack_has_language(registry, "javascript") == true, "has_language(javascript)");
    ASSERT(ts_pack_has_language(registry, "rust") == true, "has_language(rust)");
    ASSERT(ts_pack_has_language(registry, "nonexistent_xyz") == false, "has_language(nonexistent) == false");

    /* Parse Python code */
    const char *py_code = "def hello(): pass\n";
    TsPackTree *tree = ts_pack_parse_string(registry, "python", py_code, strlen(py_code));
    ASSERT(tree != NULL, "parse_string(python) returns tree");

    /* Tree properties */
    char *node_type = ts_pack_tree_root_node_type(tree);
    ASSERT(node_type != NULL && strcmp(node_type, "module") == 0, "root node type == module");
    ts_pack_free_string(node_type);

    uint32_t child_count = ts_pack_tree_root_child_count(tree);
    ASSERT(child_count >= 1, "root child count >= 1");

    bool has_errors = ts_pack_tree_has_error_nodes(tree);
    ASSERT(has_errors == false, "no error nodes in valid code");

    ts_pack_tree_free(tree);

    /* Invalid language returns NULL */
    const char *bad_code = "code";
    TsPackTree *bad_tree = ts_pack_parse_string(registry, "nonexistent_xyz_123", bad_code, strlen(bad_code));
    ASSERT(bad_tree == NULL, "parse_string(invalid) returns NULL");

    /* ====== PROCESS TESTS (4) ====== */
    printf("\n-- Process API Tests --\n");

    /* Process Python function */
    const char *py_config = "{\"language\":\"python\",\"structure\":true}";
    const char *py_source = "def hello():\n    pass\n";
    char *result = ts_pack_process(registry, py_source, strlen(py_source), py_config);
    ASSERT(result != NULL, "process() returns JSON for Python");
    ASSERT(strstr(result, "\"language\":\"python\"") != NULL, "process result contains language");
    ts_pack_free_string(result);

    /* Process JavaScript function */
    const char *js_config = "{\"language\":\"javascript\",\"structure\":true}";
    const char *js_source = "function test() { return 1; }\n";
    result = ts_pack_process(registry, js_source, strlen(js_source), js_config);
    ASSERT(result != NULL, "process() returns JSON for JavaScript");
    ASSERT(strstr(result, "\"language\":\"javascript\"") != NULL, "process result contains JS language");
    ts_pack_free_string(result);

    /* Process Rust function */
    const char *rs_config = "{\"language\":\"rust\",\"structure\":true}";
    const char *rs_source = "fn main() {\n    println!(\"hello\");\n}\n";
    result = ts_pack_process(registry, rs_source, strlen(rs_source), rs_config);
    ASSERT(result != NULL, "process() returns JSON for Rust");
    ASSERT(strstr(result, "\"language\":\"rust\"") != NULL, "process result contains Rust language");
    ts_pack_free_string(result);

    /* Process with imports */
    const char *py_import_config = "{\"language\":\"python\",\"structure\":true,\"imports\":true}";
    const char *py_import_source = "import os\nfrom sys import argv\n\ndef main():\n    pass\n";
    result = ts_pack_process(registry, py_import_source, strlen(py_import_source), py_import_config);
    ASSERT(result != NULL, "process() with imports returns JSON");
    ASSERT(strstr(result, "\"imports\"") != NULL, "process result includes imports");
    ts_pack_free_string(result);

    /* ====== CHUNKING TESTS (2) ====== */
    printf("\n-- Chunking Tests --\n");

    /* Python chunking */
    const char *chunking_config = "{\"language\":\"python\",\"structure\":true,\"chunk_max_size\":30}";
    const char *multi_func = "def a():\n    pass\n\ndef b():\n    pass\n\ndef c():\n    pass\n";
    result = ts_pack_process(registry, multi_func, strlen(multi_func), chunking_config);
    ASSERT(result != NULL, "chunking produces JSON");
    ASSERT(strstr(result, "\"chunks\"") != NULL, "chunking result includes chunks array");
    ts_pack_free_string(result);

    /* JavaScript chunking */
    const char *js_chunking = "{\"language\":\"javascript\",\"structure\":true,\"chunk_max_size\":30}";
    const char *js_multi_func = "function a() { return 1; }\n\nfunction b() { return 2; }\n\nfunction c() { return 3; }\n";
    result = ts_pack_process(registry, js_multi_func, strlen(js_multi_func), js_chunking);
    ASSERT(result != NULL, "JS chunking produces JSON");
    ASSERT(strstr(result, "\"chunks\"") != NULL, "JS chunking result includes chunks");
    ts_pack_free_string(result);

    /* ====== DOWNLOAD API TESTS (4) ====== */
    printf("\n-- Download API Tests --\n");

    /* Manifest languages */
    size_t manifest_count = 0;
    const char *const *manifest = ts_pack_manifest_languages(&manifest_count);
    ASSERT(manifest != NULL, "manifest_languages returns array");
    ASSERT(manifest_count > 50, "manifest_languages has 50+ languages");

    /* Check for expected languages in manifest */
    bool has_python_in_manifest = false;
    for (size_t i = 0; i < manifest_count; i++) {
        if (strcmp(manifest[i], "python") == 0) {
            has_python_in_manifest = true;
            break;
        }
    }
    ASSERT(has_python_in_manifest, "manifest contains 'python'");
    ts_pack_free_string_array(manifest);

    /* Downloaded languages (should be empty or small initially) */
    size_t dl_count = 0;
    const char *const *downloaded = ts_pack_downloaded_languages(&dl_count);
    ASSERT(downloaded != NULL, "downloaded_languages returns array");
    ASSERT(dl_count >= 0, "downloaded_languages count is non-negative");
    ts_pack_free_string_array(downloaded);

    /* Cache directory */
    char *cache = ts_pack_cache_dir();
    ASSERT(cache != NULL, "cache_dir returns non-null path");
    ASSERT(strlen(cache) > 0, "cache_dir path is non-empty");
    ts_pack_free_string(cache);

    /* Configure with custom cache (no download) */
    const char *config_json = "{\"cache_dir\":\"/tmp/ts_pack_test\"}";
    int32_t config_result = ts_pack_configure(config_json);
    ASSERT(config_result == 0 || config_result == -1, "configure() returns 0 or -1");

    /* ====== ERROR HANDLING TESTS (3) ====== */
    printf("\n-- Error Handling Tests --\n");

    /* Parse with error nodes */
    const char *broken_code = "def broken(:\n";
    TsPackTree *error_tree = ts_pack_parse_string(registry, "python", broken_code, strlen(broken_code));
    ASSERT(error_tree != NULL, "parse_string with syntax error returns tree");
    bool tree_has_errors = ts_pack_tree_has_error_nodes(error_tree);
    ASSERT(tree_has_errors == true, "tree has error nodes for invalid code");

    uintptr_t error_node_count = ts_pack_tree_error_count(error_tree);
    ASSERT(error_node_count > 0, "error_count returns positive for invalid code");

    char *sexp = ts_pack_tree_to_sexp(error_tree);
    ASSERT(sexp != NULL, "tree_to_sexp returns s-expression");
    ASSERT(strlen(sexp) > 0, "s-expression is non-empty");
    ts_pack_free_string(sexp);

    ts_pack_tree_free(error_tree);

    /* Cleanup */
    ts_pack_registry_free(registry);

    /* Clear error */
    ts_pack_clear_error();
    const char *err = ts_pack_last_error();
    ASSERT(err == NULL, "last_error is NULL after clear_error");

    printf("\n%s (%d failures)\n", failures == 0 ? "All tests passed!" : "Some tests failed!", failures);
    return failures == 0 ? EXIT_SUCCESS : EXIT_FAILURE;
}
