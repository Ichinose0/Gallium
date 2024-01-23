use gear::{InstanceBuilder,CommandPoolDescriptor,CommandRecorderDescriptor};

fn main() {
    let instance = InstanceBuilder::new().build();
    let connecters = instance.enumerate_connecters();
    let mut index = 0;
    let mut found_device = false;
    for i in &connecters {
        let properties = i.get_queue_family_properties();
        for i in properties {
            if i.is_compute_support() {
                index = 0;
                found_device = true;
                break;
            }
        }
    }
    if !found_device {
        panic!("No suitable device found.")
    }

    let connecter = connecters[index];

    let device = connecter.create_device(index);
    let queue = device.get_queue(index);
    let desc = CommandPoolDescriptor::new().queue_family_index(index);
    let pool = device.create_command_pool(&desc);
    let desc = CommandRecorderDescriptor::new();
    let recorders = device.allocate_command_recorder(pool,&desc);
}