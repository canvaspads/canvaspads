const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const loop_mod = b.addModule("appwayloop", .{
        .root_source_file = b.path("src/libs/loop.zig"),
        .target = target,
        .optimize = optimize,
    });
    const loop_tests = b.addTest(.{
        .root_module = loop_mod,
    });
    const run_loop_tests = b.addRunArtifact(loop_tests);

    const test_step = b.step("test", "Run tests");
    const libappwayloop = b.addLibrary(.{
        .name = "appwayloop",
        .linkage = .static,
        .root_module = loop_mod,
    });

    b.installArtifact(libappwayloop);

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/demo.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    exe.root_module.linkLibrary(libappwayloop);

    const run_step = b.step("run", "Run the app");

    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    test_step.dependOn(&run_loop_tests.step);
}
