const std = @import("std");

extern fn a() void;

pub fn main() !void {
    a();
}
