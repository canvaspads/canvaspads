const backend = @import("backend.zig");
const Adapter = @import("adapter.zig").Adapter;
const config = @import("config");

pub const Instance = struct {
    imp: backend.InstanceImpl,

    pub fn new() Instance {
        return .{
            .imp = undefined,
        };
    }

    pub fn createAdapter(self: *@This()) Adapter {
        const imp = self.imp.vtable.createAdapterImpl();
        return .{
            .imp = imp
        }
    }
};
