use {
    crate::host::dbt::interpret::interpret,
    alloc::vec::Vec,
    byteorder::ByteOrder,
    common::{hashmap::HashMap, intern::InternedString, rudder::Model},
    core::{any::type_name, borrow::Borrow, cell::UnsafeCell, ptr::NonNull},
    itertools::Itertools,
    paste::paste,
};

pub const GLOBAL_REGISTER_SIZE: usize = 2 * 1024 * 1024;

pub struct RegisterFile {
    inner: UnsafeCell<Vec<u8>>,
    registers: HashMap<InternedString, (usize, usize)>, // offset, size

    // sorted vec of all register start offsets, each pair forms a half-open range(?)
    register_offsets: Vec<usize>,

    global_register_offset: usize,
}

// todo: bad bad bad
unsafe impl Send for RegisterFile {}
unsafe impl Sync for RegisterFile {}

impl RegisterFile {
    pub fn init<M: Borrow<Model>>(model: M) -> Self {
        let model = model.borrow();

        let global_register_offset = (model.register_file_size() as usize).next_multiple_of(8);

        let size = global_register_offset + GLOBAL_REGISTER_SIZE;

        let register_file = Self {
            inner: UnsafeCell::new(alloc::vec![0u8; size]),
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

            register_offsets: model
                .registers()
                .values()
                .map(|desc| usize::try_from(desc.offset).unwrap())
                .sorted()
                .collect(),

            global_register_offset,
        };

        interpret(model, "borealis_register_init", &[], &register_file);
        configure_features(&register_file);
        interpret(model, "__InitSystem", &[], &register_file);

        register_file
    }

    pub fn global_register_offset(&self) -> usize {
        self.global_register_offset
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        unsafe { self.inner.as_mut_unchecked() }.as_mut_ptr()
    }

    pub fn get_register_size(&self, name: impl Into<InternedString>) -> Option<usize> {
        self.registers.get(&(name.into())).map(|(_, size)| *size)
    }

    pub fn write<V: RegisterValue>(&self, name: impl Into<InternedString>, value: V) {
        let name = name.into();
        let (offset, size) = self
            .registers
            .get(&name)
            .copied()
            .unwrap_or_else(|| panic!("failed to find register with name {name:?}"));

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

    pub fn write_raw<V: RegisterValue>(&self, offset: usize, value: V) {
        self.validate_range::<V>(offset);
        value.write(&mut unsafe { self.inner.as_mut_unchecked() }[offset..offset + V::SIZE]);
    }

    pub fn read<V: RegisterValue>(&self, name: impl Into<InternedString>) -> V {
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
        self.validate_range::<V>(offset);
        V::read(&unsafe { self.inner.as_ref_unchecked() }[offset..offset + V::SIZE])
    }

    pub fn as_wellknown<V: RegisterValue>(
        &self,
        name: impl Into<InternedString>,
    ) -> WellKnownRegister<V::Inner> {
        let name = name.into();
        let (offset, size) = self.registers.get(&name).copied().unwrap();

        if V::SIZE > size {
            log::error!(
                "wrong size instantiation of {name:?}: expected {:#x} got {:#x}
        ({})",
                size,
                V::SIZE,
                type_name::<V>()
            );
        }

        self.validate_range::<V>(offset);
        //V::read(&unsafe { self.inner.as_ref_unchecked() }[offset..offset + V::SIZE])

        WellKnownRegister::<V::Inner>::new(unsafe {
            NonNull::new_unchecked(
                self.inner
                    .as_mut_unchecked()
                    .as_mut_ptr()
                    .offset(offset as isize) as *mut V::Inner,
            )
        })
    }

    pub fn validate_range<V: RegisterValue>(&self, offset: usize) {
        // given an offset into the register file, return the range in which it
        // lies

        let (Ok(start_index) | Err(start_index)) =
            self.register_offsets.as_slice().binary_search(&offset);

        let Some(end) = self.register_offsets.get(start_index + 1) else {
            // writing to last register in the register file
            return;
        };

        if offset + V::SIZE > *end {
            panic!(
                "writing a {} ({} bytes) at offset {offset:#x} goes past beginning of adjacent register at offset {end:#x}",
                type_name::<V>(),
                V::SIZE,
            )
        }
    }
}

#[derive(Copy, Clone)]
pub struct WellKnownRegister<T>(NonNull<T>);

unsafe impl<T> Send for WellKnownRegister<T> {}
unsafe impl<T> Sync for WellKnownRegister<T> {}

impl<T> WellKnownRegister<T> {
    pub fn new(ptr: NonNull<T>) -> Self {
        Self(ptr)
    }

    pub fn read(&self) -> T {
        unsafe { self.0.read() }
    }

    pub fn write(&self, value: T) {
        unsafe {
            self.0.write(value);
        }
    }
}

pub trait RegisterValue {
    /// Size in bytes
    const SIZE: usize;
    type Inner;

    fn write(&self, dest: &mut [u8]);
    fn read(src: &[u8]) -> Self;
}

macro_rules! impl_register_value {
    ($target_type:ty) => {
        impl RegisterValue for $target_type {
            const SIZE: usize = (Self::BITS / 8) as usize;
            type Inner = $target_type;

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
    type Inner = u8;

    fn write(&self, dest: &mut [u8]) {
        dest[0] = *self;
    }

    fn read(src: &[u8]) -> Self {
        src[0]
    }
}

impl RegisterValue for bool {
    const SIZE: usize = 1;
    type Inner = bool;

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

fn configure_features(register_file: &RegisterFile) {
    let disabled = [
        "FEAT_LSE2_IMPLEMENTED",
        "FEAT_TME_IMPLEMENTED",
        "FEAT_BTI_IMPLEMENTED",
        "FEAT_PAuth_IMPLEMENTED",
        "FEAT_PAuth2_IMPLEMENTED",
        "FEAT_FGT_IMPLEMENTED",
        "FEAT_FGT2_IMPLEMENTED",
    ];

    disabled.into_iter().for_each(|name| {
        register_file.write(name, 0u8);
    });
}
