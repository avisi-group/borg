use {
    crate::host::objects::{
        Object, ObjectId, ObjectStore, ToDevice, ToIrqController, ToMemoryMappedDevice,
        ToRegisterMappedDevice, ToTickable,
        device::{Device, DeviceFactory, MemoryMappedDevice},
        irq::IrqController,
    },
    alloc::{collections::BTreeMap, sync::Arc, vec::Vec},
    common::intern::InternedString,
    log::error,
    proc_macro_lib::guest_device_factory,
    spin::Mutex,
};

#[guest_device_factory(a9gic)]
fn create_gic(_config: &BTreeMap<InternedString, InternedString>) -> Arc<dyn Device> {
    Arc::new(GlobalInterruptController::new())
}

// Interrupt Controller Type Register
const GICD_TYPER: u32 = 0b100_000_00111;

// CPU Interface Identification Register
// 0x0002 = GICv2, 0x043B = Pretend to be ARM implementation (JEP106 code).
const GICC_IIDR: u32 = 0x0002_043B;

const PENDING_NONE: u32 = 0x0000_03ff;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InterruptId {
    PMUIRQ,
    COMMIRQ,
    CTIIRQ,
    COMMRX,
    COMMTX,
    CNTP,
    CNTHP,
    CNTHPS,
    CNTPS,
    CNTV,
    CNTHV,
    CNTHVS,
    PMBIRQ,
}

impl InterruptId {
    pub fn raw(&self) -> u32 {
        // SGI Interrupts are 0-15, PPI interrupts are 16-31, and SPI interrupts have an
        // offset of 32.
        const PPI_OFFSET: u32 = 16;

        match self {
            InterruptId::CNTP => 0x0000_000d + PPI_OFFSET,
            InterruptId::CNTHP => 0x0000_000a + PPI_OFFSET,
            InterruptId::CNTV => 0x0000_000b + PPI_OFFSET,
            _ => PENDING_NONE,
        }
    }
}

#[derive(Debug)]
struct State {
    gicc_ctlr: u32,
    pending: Option<InterruptId>,
    active: Option<InterruptId>,
}

#[derive(Debug)]
struct GlobalInterruptController {
    id: ObjectId,
    state: Mutex<State>,
    lines: Mutex<Vec<IrqLine>>,
}

#[derive(Debug)]
struct IrqLine {
    raised: bool,
    enabled: bool,
    active: bool,
    pending: bool,
    priority: u8,
    cpu_mask: u8,
    config: u8,
}

impl IrqLine {
    pub fn new(config: u8, cpu_mask: u8) -> Self {
        Self {
            raised: false,
            enabled: false,
            active: false,
            pending: false,
            priority: 0,
            cpu_mask,
            config,
        }
    }

    pub fn edge_triggered(&self) -> bool {
        self.config & 2 == 2
    }

    pub fn level_triggered(&self) -> bool {
        self.config & 2 != 2
    }
}

impl GlobalInterruptController {
    fn new() -> Self {
        let mut lines = Vec::new();

        for i in 0..1020 {
            let mut config = 0u8;
            let mut cpu_mask = 0u8;

            if i < 16 {
                config |= 2u8;
            }

            if i < 32 {
                cpu_mask = 1u8;
            }

            lines.push(IrqLine::new(config, cpu_mask));
        }

        Self {
            id: ObjectId::new(),
            state: Mutex::new(State {
                gicc_ctlr: 0,
                pending: None,
                active: None,
            }),
            lines: Mutex::new(lines),
        }
    }

    fn acknowledge_interrupt(&self) -> u32 {
        let mut guard = self.state.lock();
        match guard.pending {
            Some(intid) => {
                guard.pending = None;
                guard.active = Some(intid);
                intid.raw()
            }
            None => PENDING_NONE,
        }
    }

    fn clear_active_interrupt(&self, interrupt_id: u32) {
        let mut guard = self.state.lock();
        if let Some(active_intid) = guard.active {
            if active_intid.raw() == interrupt_id {
                guard.active = None;
            }
        }
    }
}

impl Object for GlobalInterruptController {
    fn id(&self) -> ObjectId {
        self.id
    }
}

impl Device for GlobalInterruptController {
    fn start(&self) {}
    fn stop(&self) {}
}

impl ToTickable for GlobalInterruptController {}
impl ToRegisterMappedDevice for GlobalInterruptController {}

impl MemoryMappedDevice for GlobalInterruptController {
    fn address_space_size(&self) -> u64 {
        0x3000
    }

    /// Read `value.len()` bytes from the device starting at `offset`
    fn read(&self, offset: u64, value: &mut [u8]) {
        let response = match offset {
            // ***** Distributor Interface *****
            0x1004 => GICD_TYPER,

            // Send all interrupts to CPU interface 0
            0x1800 => 0xffffffff,

            0x1C04 => {
                // prerr_bits("[GIC] Read 1C04: ", gic_read_ram(0x1C04));
                // self.read_ram()  gic_read_ram(0x1C04) // Linux timer
                0
            }

            // ***** CPU Interface 0 *****
            0x2000 => {
                //prerr_bits("[GIC] Read GICC_CTLR ", GICC_CTLR.bits);
                self.state.lock().gicc_ctlr
            }
            0x200C => {
                let intid = self.acknowledge_interrupt();
                //prerr_bits("[GIC] Acknowledged interrupt ", intid);
                intid
            }

            0x20FC => GICC_IIDR,

            _ => {
                error!("[GIC] Read offset: {offset:x}");
                0
            }
        };

        value[..4].copy_from_slice(&response.to_le_bytes());
    }

    /// Write `value` bytes into the device starting at `offset`
    fn write(&self, offset: u64, value: &[u8]) {
        let data = u32::from_ne_bytes(value.try_into().unwrap());

        match offset {
            // ***** Distributor Interface *****
            0x1004 => readonly(offset),

            0x1100 => {
                //  prerr_bits("[GIC] Registering interrupts ", data);
                let int_id = highest_set_bit(data);
                log::error!("[GIC] Registering interrupt {int_id}")
            }

            0x1800 => readonly(offset),

            // ***** CPU Interface 0 *****
            0x2000 => {
                // prerr_bits("[GIC] GICC_CTLR = ", data);
                self.state.lock().gicc_ctlr = data;
            }
            0x200C => readonly(offset),
            0x20FC => readonly(offset),

            0x2010 => {
                //  prerr_bits("[GIC] End of interrupt = ", data);
                self.clear_active_interrupt(data);
            }

            0x3000 => {
                //  prerr_bits("[GIC] Deactivating interrupt ", data);
                self.clear_active_interrupt(data);
            }

            // We don't exhaustively model the GIC, so log and forward unrecognised writes to memory
            _ => {
                //  prerr_bits("[GIC] Unknown write offset: ", offset);
                // prerr_bits("[GIC] Unknown write data: ", data);
                error!("[GIC] write offset: {offset:x}, data: {value:x?}");
            }
        }
    }
}

impl IrqController for GlobalInterruptController {
    fn raise(&self, line: usize) {
        //error!("[GIC] raise irq {line}");

        let line = &mut self.lines.lock()[line];

        if !line.raised {
            line.raised = true;
            if line.edge_triggered() {
                line.pending = true;
            }

            // TODO: Update CPU Interface
        }
    }

    fn rescind(&self, line: usize) {
        let line = &mut self.lines.lock()[line];

        if line.raised {
            line.raised = false;
            // TODO: Update CPU Interface
        }
    }

    fn acknowledge(&self, line: usize) {
        error!("[GIC] acknowledge irq {line}");
        todo!()
    }
}

fn readonly(offset: u64) {
    error!("wrote to read-only register @ {offset:x}")
}

fn highest_set_bit(data: u32) -> u32 {
    32 - data.leading_zeros()
}
