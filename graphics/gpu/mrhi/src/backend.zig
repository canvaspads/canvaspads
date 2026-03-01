const std = @import("std");
const Allocator = std.mem.Allocator;

pub const BackendType = enum {
    vk,
};

pub const AdapterImpl = struct {
    init: *fn (Allocator) void,
    deinit: *fn (Allocator) void,
};

pub const InstanceImplVt = struct {
    init: *fn (Allocator) void,
    deinit: *fn (Allocator) void,
    createAdapterImpl: *fn (Allocator) void,
};

pub const InstanceImpl = struct {
    vtable: InstanceImplVt,
    data: *const anyopaque,
};
