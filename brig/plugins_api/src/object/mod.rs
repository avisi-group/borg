use {
    crate::object::{
        device::{Device, MemoryMappedDevice, RegisterMappedDevice},
        irq::IrqController,
        tickable::Tickable,
    },
    alloc::{fmt, sync::Arc},
    core::{
        any::Any,
        fmt::Display,
        sync::atomic::{AtomicU64, Ordering},
    },
};

pub mod device;
pub mod irq;
pub mod tickable;

static OBJECT_ID_GENERATOR: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ObjectId(u64);

impl Display for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Object({:x})", self.0)
    }
}

impl ObjectId {
    pub fn new() -> Self {
        Self(OBJECT_ID_GENERATOR.fetch_add(1, Ordering::Relaxed))
    }
}

pub trait Object:
    Send + Sync + ToDevice + ToMemoryMappedDevice + ToRegisterMappedDevice + ToTickable + Any
{
    fn id(&self) -> ObjectId;
}

pub trait ObjectStore {
    fn insert(&self, object: Arc<dyn Object>);
    fn get(&self, id: ObjectId) -> Option<Arc<dyn Object>>;
    fn get_device(&self, id: ObjectId) -> Option<Arc<dyn Device>>;
    fn get_memory_mapped_device(&self, id: ObjectId) -> Option<Arc<dyn MemoryMappedDevice>>;
    fn get_register_mapped_device(&self, id: ObjectId) -> Option<Arc<dyn RegisterMappedDevice>>;
    fn get_tickable(&self, id: ObjectId) -> Option<Arc<dyn Tickable>>;
}

pub trait ToDevice {
    fn to_device<'a>(self: Arc<Self>) -> Option<Arc<dyn Device + 'a>>
    where
        Self: 'a,
    {
        None
    }
}

impl<T: Device> ToDevice for T {
    fn to_device<'a>(self: Arc<Self>) -> Option<Arc<dyn Device + 'a>>
    where
        Self: 'a,
    {
        Some(self)
    }
}

pub trait ToMemoryMappedDevice {
    fn to_memory_mapped_device<'a>(self: Arc<Self>) -> Option<Arc<dyn MemoryMappedDevice + 'a>>
    where
        Self: 'a,
    {
        None
    }
}

impl<T: MemoryMappedDevice> ToMemoryMappedDevice for T {
    fn to_memory_mapped_device<'a>(self: Arc<Self>) -> Option<Arc<dyn MemoryMappedDevice + 'a>>
    where
        Self: 'a,
    {
        Some(self)
    }
}

pub trait ToRegisterMappedDevice {
    fn to_register_mapped_device<'a>(self: Arc<Self>) -> Option<Arc<dyn RegisterMappedDevice + 'a>>
    where
        Self: 'a,
    {
        None
    }
}

impl<T: RegisterMappedDevice> ToRegisterMappedDevice for T {
    fn to_register_mapped_device<'a>(self: Arc<Self>) -> Option<Arc<dyn RegisterMappedDevice + 'a>>
    where
        Self: 'a,
    {
        Some(self)
    }
}

pub trait ToTickable {
    fn to_tickable<'a>(self: Arc<Self>) -> Option<Arc<dyn Tickable + 'a>>
    where
        Self: 'a,
    {
        None
    }
}

impl<T: Tickable> ToTickable for T {
    fn to_tickable<'a>(self: Arc<Self>) -> Option<Arc<dyn Tickable + 'a>>
    where
        Self: 'a,
    {
        Some(self)
    }
}

pub trait ToIrqController {
    fn to_irq_controller<'a>(self: Arc<Self>) -> Option<Arc<dyn IrqController + 'a>>
    where
        Self: 'a,
    {
        None
    }
}

impl<T: IrqController> ToIrqController for T {
    fn to_irq_controller<'a>(self: Arc<Self>) -> Option<Arc<dyn IrqController + 'a>>
    where
        Self: 'a,
    {
        Some(self)
    }
}

// macro_rules! object_type {
//     ($type_name:ident, $to_name:ident) => {
//         pub trait concat_idents!(To, $type_name) {
//             fn to_$to_name<'a>(self: Arc<Self>) -> Option<Arc<dyn $type_name
// + 'a>>             where
//                 Self: 'a,
//             {
//                 None
//             }
//         }

//         impl<T: $type_name> To$type_name for T {
//             fn to_$to_name<'a>(self: Arc<Self>) -> Option<Arc<dyn $type_name
// + 'a>>             where
//                 Self: 'a,
//             {
//                 Some(self)
//             }
//         }
//     };
// }

// object_type!(IrqController, irq_controller);
