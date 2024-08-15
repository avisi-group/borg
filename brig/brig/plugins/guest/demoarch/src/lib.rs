#![no_std]

extern crate alloc;

use {
    alloc::{boxed::Box, collections::BTreeMap, string::String, sync::Arc},
    core::fmt::{self, Debug},
    plugins_rt::api::{
        guest::{
            dbt::emitter::{Block, Builder, Context, LoweringContext, Type},
            Device, DeviceFactory, Environment,
        },
        PluginHeader, PluginHost,
    },
};

#[no_mangle]
#[link_section = ".plugin_header"]
pub static PLUGIN_HEADER: PluginHeader = PluginHeader {
    name: "demoarch",
    entrypoint,
};

fn entrypoint(host: &'static dyn PluginHost) {
    plugins_rt::init(host);

    plugins_rt::get_host().register_device("demoarch", Box::new(DemoArchFactory));
    log::info!("loaded demo architecture");
}

#[derive(Debug)]
struct DemoArchFactory;

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
        let mut ctx = Context::new();

        let b0 = ctx.create_block();
        let b1 = ctx.create_block();

        let mut builder = Builder::new(b0);

        // B0
        let c0 = builder.const_u32(0);
        let c1 = builder.const_u32(1);
        let c2 = builder.read_register(c0, Type::u32());
        let c3 = builder.read_register(c1, Type::u32());
        let c4 = builder.add(c2, c3);
        let c5 = builder.const_u32(2);
        builder.write_register(c5, c4);
        builder.jump(Block::downgrade(&b1));

        // B1
        builder.set_insert_point(b1);
        builder.leave();

        ctx.lower(self.env.lowering_ctx());

        panic!("beep");
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
