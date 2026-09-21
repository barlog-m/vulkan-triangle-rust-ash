use std::sync::Arc;
use ash::vk;
use vk_mem::Alloc;

fn main() {
    let entry = ash::Entry::linked();

    let app_info = vk::ApplicationInfo::default()
        .api_version(vk::API_VERSION_1_3);

    let instance_create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info);

    let instance = unsafe { entry.create_instance(&instance_create_info, None) }.unwrap();

    let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];

    // Minimal device
    let queue_priority = [1.0f32];
    let queue_info = vk::DeviceQueueCreateInfo::default()
        .queue_family_index(0)
        .queue_priorities(&queue_priority);
    let queue_infos = [queue_info];
    let device_create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_infos);

    let device = unsafe { instance.create_device(physical_device, &device_create_info, None) }.unwrap();

    let allocator = unsafe { vk_mem::Allocator::new(vk_mem::AllocatorCreateInfo::new(
        &instance,
        &device,
        physical_device,
    )) }.unwrap();

    let device = Arc::new(device);
    let allocator = Arc::new(allocator);

    // Create a simple buffer
    let buffer_info = vk::BufferCreateInfo::default()
        .size(1024)
        .usage(vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST);

    let alloc_info = vk_mem::AllocationCreateInfo {
        usage: vk_mem::MemoryUsage::AutoPreferDevice,
        ..Default::default()
    };

    let (buffer, mut allocation) = unsafe { allocator.create_buffer(&buffer_info, &alloc_info) }.unwrap();

    println!("Created buffer, allocation valid: {}", allocator.get_allocation_info(&allocation).size);

    // Drop everything
    unsafe { allocator.destroy_buffer(buffer, &mut allocation) };

    // Now drop the Arcs
    drop(allocator);
    // We need to drop the device after allocator, but we have an Arc...
    // In Rust, Arc::drop just decrements. If we are the only owner, it drops.
    // But 'device' is still in scope.
    
    // Let's try explicit drop
    drop(Arc::try_unwrap(device).map_err(|_| "Failed to unwrap device").unwrap());

    println!("Dropped device");

    unsafe { instance.destroy_instance(None) };
    println!("Done.");
}
