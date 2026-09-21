# Vulkan Triangle in Rust

This is an example that draws a rectangle in Vulkan.

It draws a rectangle instead of a triangle because it uses shaders from the official [Vulkan
example](https://docs.vulkan.org/tutorial/latest/04_Vertex_buffers/03_Index_buffer.html).

The example targets Vulkan 1.4, uses dynamic rendering, synchronization 2, unified image layout, and enables a bunch of
unnecessary stuff just because why not.

The example uses [Vulkan Memory Allocator](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator), because nobody wants to allocate memory manually.

It also separates the code into modules and functions instead of providing you with a one-billion-lines single file.

The example uses [Ash](https://github.com/ash-rs/ash) from git, not a release, because the latest Ash release targets prehistoric Vulkan 1.3.

Also, this example bundles the [vk-mem](https://github.com/gwihlidal/vk-mem-rs) crate source code, because it had to be modified to work with Ash from git.

AI was used to make this code exists, but good luck to write same code only with AI.
