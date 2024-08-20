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
    fn create(
        &self,
        _config: BTreeMap<String, String>,
        _env: Box<dyn Environment>,
    ) -> Arc<dyn Device> {
        Arc::new(DemoArch {})
    }
}

struct DemoArch {}

impl Debug for DemoArch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "DemoArch")
    }
}

impl Device for DemoArch {
    fn start(&self) {
        // let mut ctx = X86TranslationContext::new();
        // let b0 = ctx.create_block();
        // let emitter = ctx.emitter();

        // {
        //     let reg_offset = emitter.constant(
        //         0x1234,
        //         Type {
        //             kind: TypeKind::Unsigned,
        //             width: 32,
        //         },
        //     );

        //     let reg_value = emitter.read_register(
        //         reg_offset.clone(),
        //         Type {
        //             kind: TypeKind::Unsigned,
        //             width: 32,
        //         },
        //     );

        //     let one = emitter.constant(
        //         1,
        //         Type {
        //             kind: TypeKind::Unsigned,
        //             width: 32,
        //         },
        //     );

        //     let sum = emitter.add(reg_value, one);

        //     let _ = emitter.write_register(reg_offset, sum);

        //     emitter.jump(b0.clone());
        // }

        // emitter.set_current_block(b0);
        // emitter.leave();

        // let translation = ctx.compile();

        // log::debug!("{:?}", translation);

        // let ptr = translation.code.as_ptr();
        // log::debug!("executing @ {ptr:p}");

        // unsafe {
        //     let func: extern "C" fn() = core::mem::transmute(ptr);
        //     func();
        // }
        let mut ctx = X86TranslationContext::new();
        let b1 = ctx.create_block();
        let b2 = ctx.create_block();
        let b3 = ctx.create_block();
        let emitter = ctx.emitter();

        {
            let _5 = emitter.constant(
                5,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            let _0 = emitter.constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            emitter.write_register(_0, _5);
        }

        {
            let _10 = emitter.constant(
                10,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            let _8 = emitter.constant(
                8,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            emitter.write_register(_8, _10);
        }

        {
            let _0 = emitter.constant(
                0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            let read_0 = emitter.read_register(
                _0,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );

            let _8 = emitter.constant(
                8,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            let read_8 = emitter.read_register(
                _8,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );

            let sum = emitter.add(read_0, read_8);

            let _16 = emitter.constant(
                16,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );

            emitter.write_register(_16, sum);
        }
        {
            let _16 = emitter.constant(
                16,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            let read_16 = emitter.read_register(
                _16,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );

            emitter.branch(read_16, b1.clone(), b2.clone());
        }

        {
            emitter.set_current_block(b1);
            let _20 = emitter.constant(
                20,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            let _32 = emitter.constant(
                32,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            emitter.write_register(_32, _20);
            emitter.jump(b3.clone());
        }

        {
            emitter.set_current_block(b2);
            let _30 = emitter.constant(
                30,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            let _32 = emitter.constant(
                32,
                Type {
                    kind: TypeKind::Unsigned,
                    width: 64,
                },
            );
            emitter.write_register(_32, _30);
            emitter.jump(b3.clone());
        }

        {
            emitter.set_current_block(b3);
            emitter.leave();
        }

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
