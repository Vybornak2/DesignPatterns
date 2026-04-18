// Adapter pattern

// Legacy interface
struct MicroUsbCable;
impl MicroUsbCable {
    fn plug_into_micro_usb(&self) {
        println!("Micro-USB cable connected successfully.");
    }
}

// New interface
trait LightningPort {
    fn connect_lightning(&self);
}

// Adapter
struct LightningToMicroUsbAdapter {
    legacy_cable: MicroUsbCable,
}

impl LightningPort for LightningToMicroUsbAdapter {
    fn connect_lightning(&self) {
        self.legacy_cable.plug_into_micro_usb();
    }
}

// Client code
struct AndroidPhone;
impl AndroidPhone {
    // NOTE: Dispatch Choices
    // `port: &dyn LightningPort` for dynamic dispatch (@ runtime, more flexible but slightly slower)
    // `port: impl LightningPort` for static dispatch (@ compile time, faster but less flexible)
    fn charge(&self, port: &impl LightningPort) {
        println!("Phone: Attempting to charge...");
        port.connect_lightning();
    }
    // NOTE: Static Dispatch
    // generic - fn charge<P: LightningPort>(&self, port1: &P) { ... }
    // syntactic sugar - fn charge(&self, port: &impl LightningPort) { ... }
}

fn main() {
    let phone = AndroidPhone;
    let old_cable = MicroUsbCable;

    let adapter = LightningToMicroUsbAdapter {
        legacy_cable: old_cable,
    };

    phone.charge(&adapter);
}
