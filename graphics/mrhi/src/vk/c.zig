pub const vk = @cImport({
    @cInclude("vulkan/vulkan.h");
});

test {
    _ = vk;
}
