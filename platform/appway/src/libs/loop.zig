const std = @import("std");
const posix = std.posix;
const linux = std.os.linux;

pub const CustomEventHandle = struct {};

pub const SigHandle = struct { sig: u8 };

pub const FdHandle = struct { fd: i32 };

pub const Handle = union(enum) {
    fd: FdHandle,
    sig: SigHandle,
    custom: CustomEventHandle,
};

pub const Source = struct {
    handle: Handle,
    p_self: ?*@This(),
    ud: *const anyopaque,
    callback: *const fn (*const anyopaque) void,

    pub fn destroy(self: *@This(), allocator: std.mem.Allocator) void {
        if (self.p_self) |p_self| {
            const p_ptr: []Source = @ptrCast(p_self);
            allocator.free(p_ptr);
        }
    }
};

pub fn createSource(handle: Handle, ud: *const anyopaque, callback: *const fn (*const anyopaque) void) Source {
    return .{
        .handle = handle,
        .p_self = null,
        .ud = ud,
        .callback = callback,
    };
}

pub const RunLoop = struct {
    epfd: i32,
    running: bool = false,
    sources: std.ArrayList(Source) = .empty,

    pub fn new() !@This() {
        const epfd = linux.epoll_create1(0);
        return .{
            .epfd = @intCast(epfd),
        };
    }

    pub fn registerSource(self: *@This(), source: Source, allocator: std.mem.Allocator) !void {
        switch (source.handle) {
            .fd => |fd_h| {
                var data = try allocator.alloc(Source, 1);
                data[0] = source;
                const p_data: *Source = @ptrCast(data.ptr);
                data[0].p_self = p_data;
                var ev: linux.epoll_event = .{
                    .events = linux.EPOLL.IN,
                    .data = .{ .ptr = @intFromPtr(p_data) },
                };
                _ = linux.epoll_ctl(self.epfd, linux.EPOLL.CTL_ADD, fd_h.fd, &ev);
            },
            else => {},
        }
        try self.sources.append(allocator, source);
    }

    pub fn run(self: *@This(), allocator: std.mem.Allocator) !void {
        _ = allocator;
        const EVENTS_MAX = 5;

        var events: [EVENTS_MAX]linux.epoll_event = undefined;

        self.running = true;
        while (self.running) {
            const n_fd = linux.epoll_wait(self.epfd, &events, EVENTS_MAX, -1);
            if (n_fd < 0) {
                break;
            }
            for (0..n_fd) |i| {
                const p_source: ?*const Source = @ptrFromInt(events[i].data.ptr);
                if (p_source) |s| {
                    s.callback(s.ud);
                }
            }
        }
    }

    pub fn exit(self: *@This(), allocator: std.mem.Allocator) void {
        std.log.info("exit", .{});
        for (self.sources.items) |*source| {
            source.destroy(allocator);
        }
        self.sources.deinit(allocator);
    }
};

fn cb(ud: *const anyopaque) void {
    _ = ud;

    var buf: [512]u8 = undefined;
    const n = posix.read(posix.STDIN_FILENO, &buf) catch {
        return;
    };
    if (n > 0) {
        if (std.mem.eql(u8, buf[0 .. n - 1], "exit")) {
            std.log.info("exit", .{});
        } else {
            std.log.info("unknown command", .{});
        }
    }
}

fn v() !void {
    var gpa: std.heap.DebugAllocator(.{}) = .init;
    const allocator = gpa.allocator();
    var loop = try RunLoop.new();
    const stdin_source = createSource(.{ .fd = .{ .fd = posix.STDIN_FILENO } }, undefined, &cb);
    try loop.registerSource(stdin_source, allocator);
    try loop.run(allocator);
}

pub export fn a() void {
    v() catch {
        std.debug.print("runerr\n", .{});
        return;
    };
}
