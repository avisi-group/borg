use {
    crate::dbt::{
        emitter::{Emitter, Type},
        x86::{emitter::BinaryOperationKind, X86TranslationContext},
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
        let mut ctx = X86TranslationContext::new();
        let b1 = ctx.create_block();
        let b2 = ctx.create_block();
        let b3 = ctx.create_block();
        let emitter = ctx.emitter();

        {
            let _5 = emitter.constant(5, Type::Unsigned(64));
            let _0 = emitter.constant(0, Type::Unsigned(64));
            emitter.write_register(_0, _5);
        }

        {
            let _10 = emitter.constant(10, Type::Unsigned(64));
            let _8 = emitter.constant(8, Type::Unsigned(64));
            emitter.write_register(_8, _10);
        }

        {
            let _0 = emitter.constant(0, Type::Unsigned(64));
            let read_0 = emitter.read_register(_0, Type::Unsigned(64));

            let _8 = emitter.constant(8, Type::Unsigned(64));
            let read_8 = emitter.read_register(_8, Type::Unsigned(64));

            let sum = emitter.binary_operation(BinaryOperationKind::Add(read_0, read_8));

            let _16 = emitter.constant(16, Type::Unsigned(64));

            emitter.write_register(_16, sum);
        }
        {
            let _16 = emitter.constant(16, Type::Unsigned(64));
            let read_16 = emitter.read_register(_16, Type::Unsigned(64));

            emitter.branch(read_16, b1.clone(), b2.clone());
        }

        {
            emitter.set_current_block(b1);
            let _20 = emitter.constant(20, Type::Unsigned(64));
            let _32 = emitter.constant(32, Type::Unsigned(64));
            emitter.write_register(_32, _20);
            emitter.jump(b3.clone());
        }

        {
            emitter.set_current_block(b2);
            let _30 = emitter.constant(30, Type::Unsigned(64));
            let _32 = emitter.constant(32, Type::Unsigned(64));
            emitter.write_register(_32, _30);
            emitter.jump(b3.clone());
        }

        {
            emitter.set_current_block(b3);
            emitter.leave();
        }

        let translation = ctx.compile();

        log::debug!("\n{:?}", translation);
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

// #[ktest]
// fn aarch64_addwithcarry() {
//     use crate::guest::devices::aarch64::{
//         AddWithCarry, Bits, State, Struct188a1c3bf231c64b, Tracer,
//     };

//     struct NoneTracer;

//     impl Tracer for NoneTracer {
//         fn begin(&self, _: u32, _: u64) {}

//         fn end(&self) {}

//         fn read_register(&self, _: usize, _: &dyn Debug) {}

//         fn write_register(&self, _: usize, _: &dyn Debug) {}

//         fn read_memory(&self, _: usize, _: &dyn Debug) {}

//         fn write_memory(&self, _: usize, _: &dyn Debug) {}
//     }

//     struct NoneEnv;

//     impl Environment for NoneEnv {
//         fn read_memory(&self, _: u64, _: &mut [u8]) {}

//         fn write_memory(&self, _: u64, _: &[u8]) {}
//     }

//     let mut state = State::new(Box::new(NoneEnv));

//     let x = Bits::new(0x0, 0x40);
//     let y = Bits::new(-5i128 as u128, 0x40);
//     let carry_in = false;

//     assert_eq!(
//         AddWithCarry(&mut state, &NoneTracer, x, y, carry_in),
//         Struct188a1c3bf231c64b {
//             tuple__pcnt_bv__pcnt_bv40: Bits::new(-5i64 as u128, 0x40),
//             tuple__pcnt_bv__pcnt_bv41: 0b1000
//         }
//     );
// }
