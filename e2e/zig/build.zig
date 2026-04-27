const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");

    const download_module = b.createModule(.{
        .root_source_file = b.path("src/download_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const download_tests = b.addTest(.{
        .root_module = download_module,
    });
    const download_run = b.addRunArtifact(download_tests);
    test_step.dependOn(&download_run.step);

    const error_handling_module = b.createModule(.{
        .root_source_file = b.path("src/error_handling_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const error_handling_tests = b.addTest(.{
        .root_module = error_handling_module,
    });
    const error_handling_run = b.addRunArtifact(error_handling_tests);
    test_step.dependOn(&error_handling_run.step);

    const extraction_module = b.createModule(.{
        .root_source_file = b.path("src/extraction_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const extraction_tests = b.addTest(.{
        .root_module = extraction_module,
    });
    const extraction_run = b.addRunArtifact(extraction_tests);
    test_step.dependOn(&extraction_run.step);

    const language_detection_module = b.createModule(.{
        .root_source_file = b.path("src/language_detection_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const language_detection_tests = b.addTest(.{
        .root_module = language_detection_module,
    });
    const language_detection_run = b.addRunArtifact(language_detection_tests);
    test_step.dependOn(&language_detection_run.step);

    const parsing_module = b.createModule(.{
        .root_source_file = b.path("src/parsing_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const parsing_tests = b.addTest(.{
        .root_module = parsing_module,
    });
    const parsing_run = b.addRunArtifact(parsing_tests);
    test_step.dependOn(&parsing_run.step);

    const process_module = b.createModule(.{
        .root_source_file = b.path("src/process_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const process_tests = b.addTest(.{
        .root_module = process_module,
    });
    const process_run = b.addRunArtifact(process_tests);
    test_step.dependOn(&process_run.step);

    const query_module = b.createModule(.{
        .root_source_file = b.path("src/query_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const query_tests = b.addTest(.{
        .root_module = query_module,
    });
    const query_run = b.addRunArtifact(query_tests);
    test_step.dependOn(&query_run.step);

    const registry_module = b.createModule(.{
        .root_source_file = b.path("src/registry_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const registry_tests = b.addTest(.{
        .root_module = registry_module,
    });
    const registry_run = b.addRunArtifact(registry_tests);
    test_step.dependOn(&registry_run.step);

    const smoke_module = b.createModule(.{
        .root_source_file = b.path("src/smoke_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const smoke_tests = b.addTest(.{
        .root_module = smoke_module,
    });
    const smoke_run = b.addRunArtifact(smoke_tests);
    test_step.dependOn(&smoke_run.step);

    const tree_inspection_module = b.createModule(.{
        .root_source_file = b.path("src/tree_inspection_test.zig"),
        .target = target,
        .optimize = optimize,
    });
    const tree_inspection_tests = b.addTest(.{
        .root_module = tree_inspection_module,
    });
    const tree_inspection_run = b.addRunArtifact(tree_inspection_tests);
    test_step.dependOn(&tree_inspection_run.step);

}
