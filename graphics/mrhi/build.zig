const std = @import("std");
const builtin = @import("builtin");

fn featVkShouldEnable() bool {
    return switch (builtin.os.tag) {
        .linux => true,
        else => false,
    };
}

fn featMetalShouldEnable() bool {
    return switch (builtin.os.tag) {
        .macos => true,
        else => false,
    };
}

fn featD3D11ShouldEnable() bool {
    return switch (builtin.os.tag) {
        .windows => true,
        else => false,
    };
}

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const enable_vk = b.option(bool, "enable_vk", "enabling vulkan backend") orelse featVkShouldEnable();
    const enable_metal = b.option(bool, "enable_metal", "enabling metal backend") orelse featMetalShouldEnable();
    const enable_d3d11 = b.option(bool, "enable_d3d11", "enabling d3d11 backend") orelse featD3D11ShouldEnable();

    // Build options
    const options = b.addOptions();
    options.addOption(bool, "enable_vk", enable_vk);
    options.addOption(bool, "enable_metal", enable_metal);
    options.addOption(bool, "enable_d3d11", enable_d3d11);

    // Main module
    const mrhi_mod = b.addModule("mrhi", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .link_libc = true,
        .optimize = optimize,
    });

    mrhi_mod.addOptions("config", options);

    if (enable_vk) {
        const vk_mod = b.addModule("mrhi_vk", .{
            .root_source_file = b.path("src/vk/root.zig"),
            .target = target,
            .link_libc = true,
            .optimize = optimize,
        });

        mrhi_mod.addImport("mrhi_vk", vk_mod);
    }

    const libmrhi = b.addLibrary(.{
        .name = "mrhi",
        .linkage = .static,
        .root_module = mrhi_mod,
    });

    b.installArtifact(libmrhi);

    const exe = b.addExecutable(.{
        .name = "demo",
        .root_module = b.createModule(.{
            .root_source_file = b.path("demo.zig"),
            .target = target,
            .optimize = optimize,
        }),
    });

    const run_step = b.step("run", "Run the app");

    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const mod_tests = b.addTest(.{
        .root_module = mrhi_mod,
    });

    const run_mod_tests = b.addRunArtifact(mod_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
}
