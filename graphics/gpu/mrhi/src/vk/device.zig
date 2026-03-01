const c = @import("c.zig").vk;

pub const Device = struct {};

pub const PhysicalDeviceOption = struct {};

pub const PhysicalDevice = struct {
    pdev: c.VkPhysicalDevice,

    pub fn new(instance: c.VkInstance, option: PhysicalDeviceOption) !@This() {
        _ = instance;
        _ = option;
    }

    pub fn createDevice(self: *@This()) !Device {
        _ = self;
    }
};
