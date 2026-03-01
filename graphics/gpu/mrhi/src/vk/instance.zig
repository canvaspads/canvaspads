const c = @import("c.zig").vk;

pub const Instance = struct {};

pub fn createInstance() Instance {
    return .{
        .vt = .{},
        .instance = undefined,
    };
}

test {
    const std = @import("std");
    _ = std.testing.refAllDecls(@This());
    _ = createInstance();
}
