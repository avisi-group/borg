use {
    crate::dbt::interpret::interpret,
    alloc::{collections::BTreeMap, vec::Vec},
    byteorder::ByteOrder,
    common::{hashmap::HashMap, intern::InternedString, rudder::Model},
    core::{any::type_name, borrow::Borrow},
    paste::paste,
};

pub struct RegisterFile {
    inner: Vec<u8>,
    registers: HashMap<InternedString, (usize, usize)>, // offset, size
    registers_by_offset: BTreeMap<usize, InternedString>,
}

impl RegisterFile {
    pub fn init<M: Borrow<Model>>(model: M) -> Self {
        let model = model.borrow();

        let mut register_file = Self {
            inner: alloc::vec![0u8; model.register_file_size() as usize],
            registers: model
                .registers()
                .iter()
                .map(|(name, desc)| {
                    (
                        *name,
                        (
                            usize::try_from(desc.offset).unwrap(),
                            usize::from(desc.typ.width_bytes()),
                        ),
                    )
                })
                .collect(),
            registers_by_offset: model
                .registers()
                .iter()
                .map(|(name, desc)| (usize::try_from(desc.offset).unwrap(), *name))
                .collect(),
        };

        interpret(model, "borealis_register_init", &[], &mut register_file);
        configure_features(&mut register_file);
        interpret(model, "__InitSystem", &[], &mut register_file);

        register_file
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.inner.as_mut_ptr()
    }

    pub fn lookup(&self, offset: usize) -> InternedString {
        self.registers_by_offset
            .range(..=offset)
            .map(|(_, name)| *name)
            .next_back()
            .unwrap()
    }

    pub fn write<V: RegisterValue, S: Into<InternedString>>(&mut self, name: S, value: V) {
        let name = name.into();
        let (offset, size) = self.registers.get(&name).copied().unwrap();

        if V::SIZE > size {
            log::error!(
                "wrong size write to {name:?}: expected {:#x} got {:#x} ({})",
                size,
                V::SIZE,
                type_name::<V>()
            );
        }

        self.write_raw(offset, value)
    }

    pub fn write_raw<V: RegisterValue>(&mut self, offset: usize, value: V) {
        value.write(&mut self.inner[offset..offset + V::SIZE]);
    }

    pub fn read<V: RegisterValue, S: Into<InternedString>>(&self, name: S) -> V {
        let name = name.into();
        let (offset, size) = self.registers.get(&name).copied().unwrap();

        if V::SIZE > size {
            log::error!(
                "wrong size read of {name:?}: expected {:#x} got {:#x} ({})",
                size,
                V::SIZE,
                type_name::<V>()
            );
        }

        self.read_raw(offset)
    }

    pub fn read_raw<V: RegisterValue>(&self, offset: usize) -> V {
        V::read(&self.inner[offset..offset + V::SIZE])
    }
}

pub trait RegisterValue {
    /// Size in bytes
    const SIZE: usize;

    fn write(&self, dest: &mut [u8]);
    fn read(src: &[u8]) -> Self;
}

macro_rules! impl_register_value {
    ($target_type:ty) => {
        impl RegisterValue for $target_type {
            const SIZE: usize = (Self::BITS / 8) as usize;

            paste! {
                fn write(&self, dest: &mut [u8]) {
                    byteorder::LittleEndian::[<write_ $target_type>](dest, *self);
                }

                fn read(src: &[u8]) -> Self {
                    byteorder::LittleEndian::[<read_ $target_type>](src)
                }
            }
        }
    };
}

impl_register_value!(u128);
impl_register_value!(u64);
impl_register_value!(u32);
impl_register_value!(u16);
impl_register_value!(i64);
impl_register_value!(i32);
impl_register_value!(i16);

impl RegisterValue for u8 {
    const SIZE: usize = 1;

    fn write(&self, dest: &mut [u8]) {
        dest[0] = *self;
    }

    fn read(src: &[u8]) -> Self {
        src[0]
    }
}

impl RegisterValue for bool {
    const SIZE: usize = 1;

    fn write(&self, dest: &mut [u8]) {
        dest[0] = match self {
            true => 1,
            false => 0,
        };
    }

    fn read(src: &[u8]) -> Self {
        match src[0] {
            0 => false,
            1 => true,
            _ => unreachable!(),
        }
    }
}

fn configure_features(register_file: &mut RegisterFile) {
    let disabled = [
        "FEAT_LSE2_IMPLEMENTED",
        "FEAT_TME_IMPLEMENTED",
        "FEAT_BTI_IMPLEMENTED",
    ];

    disabled.into_iter().for_each(|name| {
        register_file.write(name, 0u8);
    });
}
