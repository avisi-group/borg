use {
    crate::dbt::{
        emitter::{Emitter, Type, TypeKind},
        x86::X86TranslationContext,
        TranslationContext,
    },
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    core::fmt::{self, Debug},
    plugins_api::guest::{Device, DeviceFactory, Environment},
};

#[derive(Debug)]
pub struct DemoArchFactory;

impl DeviceFactory for DemoArchFactory {
    fn create(&self, _: BTreeMap<String, String>, env: Box<dyn Environment>) -> Arc<dyn Device> {
        Arc::new(DemoArch { env })
    }
}

struct DemoArch {
    env: Box<dyn Environment>,
}

impl Debug for DemoArch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "DemoArch")
    }
}

impl Device for DemoArch {
    fn start(&self) {
        let mut ctx = X86TranslationContext::new();
        let b0 = ctx.create_block();
        let emitter = ctx.emitter();

        {
            let reg_offset = emitter.constant(
                0x1234,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );

            let reg_value = emitter.read_register(
                reg_offset.clone(),
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );

            let one = emitter.constant(
                1,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 32,
                },
            );

            let sum = emitter.add(reg_value, one);

            let _ = emitter.write_register(reg_offset, sum);

            emitter.jump(b0.clone());
        }

        emitter.set_current_block(b0);
        emitter.leave();

        let translation = ctx.compile();

        log::debug!("{:?}", translation);
    }

    fn stop(&self) {
        todo!()
    }

    fn address_space_size(&self) -> u64 {
        0
    }

    fn read(&self, _: u64, _: &mut [u8]) {
        unimplemented!()
    }
    fn write(&self, _: u64, _: &[u8]) {
        unimplemented!()
    }
}
