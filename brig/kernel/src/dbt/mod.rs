use {
    crate::{arch::x86::memory::VirtualMemoryArea, dbt::interpret::interpret},
    alloc::{string::String, vec::Vec},
    common::{mask::mask, rudder::Model},
    core::{
        borrow::Borrow,
        fmt::{self, Debug},
    },
    iced_x86::{Formatter, Instruction},
    x86_64::{VirtAddr, structures::paging::PageTableFlags},
};

pub mod emitter;
pub mod interpret;
pub mod models;
mod tests;
mod trampoline;
pub mod translate;
pub mod x86;

pub struct Translation {
    pub code: Vec<u8>,
}

impl Translation {
    pub fn new(code: Vec<u8>) -> Self {
        let start = VirtAddr::from_ptr(code.as_ptr());
        VirtualMemoryArea::current().update_flags_range(
            start..start + code.len() as u64,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE, // removing  "NOEXECUTE" flag
        );
        Self { code }
    }

    pub fn execute(&self, register_file: *mut u8) {
        let code_ptr = self.code.as_ptr();

        unsafe { trampoline::trampoline(code_ptr, register_file) };
    }
}

impl Drop for Translation {
    fn drop(&mut self) {
        let start = VirtAddr::from_ptr(self.code.as_ptr());
        VirtualMemoryArea::current().update_flags_range(
            start..start + self.code.len() as u64,
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
        );
    }
}

impl Debug for Translation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut decoder = iced_x86::Decoder::with_ip(64, &self.code, 0, 0);

        let mut formatter = iced_x86::GasFormatter::new();

        let mut output = String::new();

        let mut instr = Instruction::default();

        while decoder.can_decode() {
            output.clear();
            decoder.decode_out(&mut instr);
            formatter.format(&instr, &mut output);
            writeln!(f, "{:016x} {output}", instr.ip())?;
        }

        Ok(())
    }
}

fn bit_insert(target: u64, source: u64, start: u64, length: u64) -> u64 {
    // todo: hack
    if start >= 64 {
        if source == 0 {
            return 0;
        } else {
            panic!("attempting to insert {length} bits of {source} into {target} at {start}");
        }
    }

    let length = u32::try_from(length).unwrap();

    let cleared_target = {
        let mask = !(mask(length)
            .checked_shl(u32::try_from(start).unwrap())
            .unwrap_or_else(|| {
                panic!("overflow in shl with {target:b} {source:?} {start:?} {length:?}")
            }));
        target & mask
    };

    let shifted_source = {
        let mask = mask(length);
        let masked_source = source & mask;
        masked_source << start
    };

    cleared_target | shifted_source
}

fn bit_extract(value: u64, start: u64, length: u64) -> u64 {
    (value >> start) & mask(u32::try_from(length).unwrap())
}

fn init_register_file<M: Borrow<Model>>(model: M) -> Vec<u8> {
    let model = model.borrow();
    let mut register_file = alloc::vec![0u8; model.register_file_size() as usize];
    let register_file_ptr = register_file.as_mut_ptr();

    interpret(model, "borealis_register_init", &[], register_file_ptr);
    configure_features(model, register_file_ptr);
    interpret(model, "__InitSystem", &[], register_file_ptr);

    register_file
}

fn configure_features(model: &Model, register_file: *mut u8) {
    let features = [
        "FEAT_AA32EL0_IMPLEMENTED",
        "FEAT_AA32EL1_IMPLEMENTED",
        "FEAT_AA32EL2_IMPLEMENTED",
        "FEAT_AA32EL3_IMPLEMENTED",
        "FEAT_AA64EL0_IMPLEMENTED",
        "FEAT_AA64EL1_IMPLEMENTED",
        "FEAT_AA64EL2_IMPLEMENTED",
        "FEAT_AA64EL3_IMPLEMENTED",
        "FEAT_EL0_IMPLEMENTED",
        "FEAT_EL1_IMPLEMENTED",
        "FEAT_EL2_IMPLEMENTED",
        "FEAT_EL3_IMPLEMENTED",
        "FEAT_AES_IMPLEMENTED",
        "FEAT_AdvSIMD_IMPLEMENTED",
        "FEAT_CSV2_1p1_IMPLEMENTED",
        "FEAT_CSV2_1p2_IMPLEMENTED",
        "FEAT_CSV2_2_IMPLEMENTED",
        "FEAT_CSV2_3_IMPLEMENTED",
        "FEAT_DoubleLock_IMPLEMENTED",
        "FEAT_ETMv4_IMPLEMENTED",
        "FEAT_ETMv4p1_IMPLEMENTED",
        "FEAT_ETMv4p2_IMPLEMENTED",
        "FEAT_ETMv4p3_IMPLEMENTED",
        "FEAT_ETMv4p4_IMPLEMENTED",
        "FEAT_ETMv4p5_IMPLEMENTED",
        "FEAT_ETMv4p6_IMPLEMENTED",
        "FEAT_ETS2_IMPLEMENTED",
        "FEAT_FP_IMPLEMENTED",
        "FEAT_GICv3_IMPLEMENTED",
        "FEAT_GICv3_LEGACY_IMPLEMENTED",
        "FEAT_GICv3_TDIR_IMPLEMENTED",
        "FEAT_GICv3p1_IMPLEMENTED",
        "FEAT_GICv4_IMPLEMENTED",
        "FEAT_GICv4p1_IMPLEMENTED",
        "FEAT_IVIPT_IMPLEMENTED",
        "FEAT_PCSRv8_IMPLEMENTED",
        "FEAT_PMULL_IMPLEMENTED",
        "FEAT_PMUv3_IMPLEMENTED",
        "FEAT_PMUv3_EXT_IMPLEMENTED",
        "FEAT_PMUv3_EXT32_IMPLEMENTED",
        "FEAT_SHA1_IMPLEMENTED",
        "FEAT_SHA256_IMPLEMENTED",
        "FEAT_TRC_EXT_IMPLEMENTED",
        "FEAT_TRC_SR_IMPLEMENTED",
        "FEAT_nTLBPA_IMPLEMENTED",
        "FEAT_CRC32_IMPLEMENTED",
        "FEAT_Debugv8p1_IMPLEMENTED",
        "FEAT_HAFDBS_IMPLEMENTED",
        "FEAT_HPDS_IMPLEMENTED",
        "FEAT_LOR_IMPLEMENTED",
        "FEAT_LSE_IMPLEMENTED",
        "FEAT_PAN_IMPLEMENTED",
        "FEAT_PMUv3p1_IMPLEMENTED",
        "FEAT_RDM_IMPLEMENTED",
        "FEAT_VHE_IMPLEMENTED",
        "FEAT_VMID16_IMPLEMENTED",
        "FEAT_AA32BF16_IMPLEMENTED",
        "FEAT_AA32HPD_IMPLEMENTED",
        "FEAT_AA32I8MM_IMPLEMENTED",
        "FEAT_ASMv8p2_IMPLEMENTED",
        "FEAT_DPB_IMPLEMENTED",
        "FEAT_Debugv8p2_IMPLEMENTED",
        "FEAT_EDHSR_IMPLEMENTED",
        "FEAT_F32MM_IMPLEMENTED",
        "FEAT_F64MM_IMPLEMENTED",
        "FEAT_FP16_IMPLEMENTED",
        "FEAT_HPDS2_IMPLEMENTED",
        "FEAT_I8MM_IMPLEMENTED",
        "FEAT_IESB_IMPLEMENTED",
        "FEAT_LPA_IMPLEMENTED",
        "FEAT_LSMAOC_IMPLEMENTED",
        "FEAT_LVA_IMPLEMENTED",
        "FEAT_MPAM_IMPLEMENTED",
        "FEAT_PAN2_IMPLEMENTED",
        "FEAT_PCSRv8p2_IMPLEMENTED",
        "FEAT_RAS_IMPLEMENTED",
        "FEAT_SHA3_IMPLEMENTED",
        "FEAT_SHA512_IMPLEMENTED",
        "FEAT_SM3_IMPLEMENTED",
        "FEAT_SM4_IMPLEMENTED",
        "FEAT_SPE_IMPLEMENTED",
        "FEAT_SVE_IMPLEMENTED",
        "FEAT_TTCNP_IMPLEMENTED",
        "FEAT_UAO_IMPLEMENTED",
        "FEAT_VPIPT_IMPLEMENTED",
        "FEAT_XNX_IMPLEMENTED",
        "FEAT_CCIDX_IMPLEMENTED",
        "FEAT_CONSTPACFIELD_IMPLEMENTED",
        "FEAT_EPAC_IMPLEMENTED",
        "FEAT_FCMA_IMPLEMENTED",
        "FEAT_FPAC_IMPLEMENTED",
        "FEAT_FPACCOMBINE_IMPLEMENTED",
        "FEAT_JSCVT_IMPLEMENTED",
        "FEAT_LRCPC_IMPLEMENTED",
        "FEAT_NV_IMPLEMENTED",
        "FEAT_PACIMP_IMPLEMENTED",
        "FEAT_PACQARMA3_IMPLEMENTED",
        "FEAT_PACQARMA5_IMPLEMENTED",
        "FEAT_PAuth_IMPLEMENTED",
        "FEAT_SPEv1p1_IMPLEMENTED",
        "FEAT_AMUv1_IMPLEMENTED",
        "FEAT_BBM_IMPLEMENTED",
        "FEAT_CNTSC_IMPLEMENTED",
        "FEAT_DIT_IMPLEMENTED",
        "FEAT_Debugv8p4_IMPLEMENTED",
        "FEAT_DotProd_IMPLEMENTED",
        "FEAT_DoubleFault_IMPLEMENTED",
        "FEAT_FHM_IMPLEMENTED",
        "FEAT_FlagM_IMPLEMENTED",
        "FEAT_IDST_IMPLEMENTED",
        "FEAT_LRCPC2_IMPLEMENTED",
        "FEAT_LSE2_IMPLEMENTED",
        "FEAT_NV2_IMPLEMENTED",
        "FEAT_PMUv3p4_IMPLEMENTED",
        "FEAT_RASSAv1p1_IMPLEMENTED",
        "FEAT_RASv1p1_IMPLEMENTED",
        "FEAT_S2FWB_IMPLEMENTED",
        "FEAT_SEL2_IMPLEMENTED",
        "FEAT_TLBIOS_IMPLEMENTED",
        "FEAT_TLBIRANGE_IMPLEMENTED",
        "FEAT_TRF_IMPLEMENTED",
        "FEAT_TTL_IMPLEMENTED",
        "FEAT_TTST_IMPLEMENTED",
        "FEAT_BTI_IMPLEMENTED",
        "FEAT_CSV2_IMPLEMENTED",
        "FEAT_CSV3_IMPLEMENTED",
        "FEAT_DPB2_IMPLEMENTED",
        "FEAT_E0PD_IMPLEMENTED",
        "FEAT_EVT_IMPLEMENTED",
        "FEAT_ExS_IMPLEMENTED",
        "FEAT_FRINTTS_IMPLEMENTED",
        "FEAT_FlagM2_IMPLEMENTED",
        "FEAT_GTG_IMPLEMENTED",
        "FEAT_MTE_IMPLEMENTED",
        "FEAT_MTE2_IMPLEMENTED",
        "FEAT_PMUv3p5_IMPLEMENTED",
        "FEAT_RNG_IMPLEMENTED",
        "FEAT_RNG_TRAP_IMPLEMENTED",
        "FEAT_SB_IMPLEMENTED",
        "FEAT_SPECRES_IMPLEMENTED",
        "FEAT_SSBS_IMPLEMENTED",
        "FEAT_SSBS2_IMPLEMENTED",
        "FEAT_AMUv1p1_IMPLEMENTED",
        "FEAT_BF16_IMPLEMENTED",
        "FEAT_DGH_IMPLEMENTED",
        "FEAT_ECV_IMPLEMENTED",
        "FEAT_FGT_IMPLEMENTED",
        "FEAT_HPMN0_IMPLEMENTED",
        "FEAT_MPAMv0p1_IMPLEMENTED",
        "FEAT_MPAMv1p1_IMPLEMENTED",
        "FEAT_MTPMU_IMPLEMENTED",
        "FEAT_PAuth2_IMPLEMENTED",
        "FEAT_TWED_IMPLEMENTED",
        "FEAT_AFP_IMPLEMENTED",
        "FEAT_EBF16_IMPLEMENTED",
        "FEAT_HCX_IMPLEMENTED",
        "FEAT_LPA2_IMPLEMENTED",
        "FEAT_LS64_IMPLEMENTED",
        "FEAT_LS64_ACCDATA_IMPLEMENTED",
        "FEAT_LS64_V_IMPLEMENTED",
        "FEAT_MTE3_IMPLEMENTED",
        "FEAT_PAN3_IMPLEMENTED",
        "FEAT_PMUv3p7_IMPLEMENTED",
        "FEAT_RPRES_IMPLEMENTED",
        "FEAT_SPEv1p2_IMPLEMENTED",
        "FEAT_WFxT_IMPLEMENTED",
        "FEAT_XS_IMPLEMENTED",
        "FEAT_CMOW_IMPLEMENTED",
        "FEAT_Debugv8p8_IMPLEMENTED",
        "FEAT_GICv3_NMI_IMPLEMENTED",
        "FEAT_HBC_IMPLEMENTED",
        "FEAT_MOPS_IMPLEMENTED",
        "FEAT_NMI_IMPLEMENTED",
        "FEAT_PMUv3_EXT64_IMPLEMENTED",
        "FEAT_PMUv3_TH_IMPLEMENTED",
        "FEAT_PMUv3p8_IMPLEMENTED",
        "FEAT_SCTLR2_IMPLEMENTED",
        "FEAT_SPEv1p3_IMPLEMENTED",
        "FEAT_TCR2_IMPLEMENTED",
        "FEAT_TIDCP1_IMPLEMENTED",
        "FEAT_ADERR_IMPLEMENTED",
        "FEAT_AIE_IMPLEMENTED",
        "FEAT_ANERR_IMPLEMENTED",
        "FEAT_CLRBHB_IMPLEMENTED",
        "FEAT_CSSC_IMPLEMENTED",
        "FEAT_Debugv8p9_IMPLEMENTED",
        "FEAT_DoubleFault2_IMPLEMENTED",
        "FEAT_ECBHB_IMPLEMENTED",
        "FEAT_FGT2_IMPLEMENTED",
        "FEAT_HAFT_IMPLEMENTED",
        "FEAT_LRCPC3_IMPLEMENTED",
        "FEAT_MTE4_IMPLEMENTED",
        "FEAT_MTE_ASYM_FAULT_IMPLEMENTED",
        "FEAT_MTE_ASYNC_IMPLEMENTED",
        "FEAT_MTE_CANONICAL_TAGS_IMPLEMENTED",
        "FEAT_MTE_NO_ADDRESS_TAGS_IMPLEMENTED",
        "FEAT_MTE_PERM_IMPLEMENTED",
        "FEAT_MTE_STORE_ONLY_IMPLEMENTED",
        "FEAT_MTE_TAGGED_FAR_IMPLEMENTED",
        "FEAT_PCSRv8p9_IMPLEMENTED",
        "FEAT_PFAR_IMPLEMENTED",
        "FEAT_PMUv3_EDGE_IMPLEMENTED",
        "FEAT_PMUv3_ICNTR_IMPLEMENTED",
        "FEAT_PMUv3_SS_IMPLEMENTED",
        "FEAT_PMUv3p9_IMPLEMENTED",
        "FEAT_PRFMSLC_IMPLEMENTED",
        "FEAT_RASSAv2_IMPLEMENTED",
        "FEAT_RASv2_IMPLEMENTED",
        "FEAT_RPRFM_IMPLEMENTED",
        "FEAT_S1PIE_IMPLEMENTED",
        "FEAT_S1POE_IMPLEMENTED",
        "FEAT_S2PIE_IMPLEMENTED",
        "FEAT_S2POE_IMPLEMENTED",
        "FEAT_SPECRES2_IMPLEMENTED",
        "FEAT_SPE_CRR_IMPLEMENTED",
        "FEAT_SPE_FDS_IMPLEMENTED",
        "FEAT_SPEv1p4_IMPLEMENTED",
        "FEAT_SPMU_IMPLEMENTED",
        "FEAT_THE_IMPLEMENTED",
        "FEAT_DoPD_IMPLEMENTED",
        "FEAT_ETE_IMPLEMENTED",
        "FEAT_SVE2_IMPLEMENTED",
        "FEAT_SVE_AES_IMPLEMENTED",
        "FEAT_SVE_BitPerm_IMPLEMENTED",
        "FEAT_SVE_PMULL128_IMPLEMENTED",
        "FEAT_SVE_SHA3_IMPLEMENTED",
        "FEAT_SVE_SM4_IMPLEMENTED",
        "FEAT_TME_IMPLEMENTED",
        "FEAT_TRBE_IMPLEMENTED",
        "FEAT_ETEv1p1_IMPLEMENTED",
        "FEAT_BRBE_IMPLEMENTED",
        "FEAT_ETEv1p2_IMPLEMENTED",
        "FEAT_RME_IMPLEMENTED",
        "FEAT_SME_IMPLEMENTED",
        "FEAT_SME_F64F64_IMPLEMENTED",
        "FEAT_SME_FA64_IMPLEMENTED",
        "FEAT_SME_I16I64_IMPLEMENTED",
        "FEAT_BRBEv1p1_IMPLEMENTED",
        "FEAT_MEC_IMPLEMENTED",
        "FEAT_SME2_IMPLEMENTED",
        "FEAT_ABLE_IMPLEMENTED",
        "FEAT_CHK_IMPLEMENTED",
        "FEAT_D128_IMPLEMENTED",
        "FEAT_EBEP_IMPLEMENTED",
        "FEAT_ETEv1p3_IMPLEMENTED",
        "FEAT_GCS_IMPLEMENTED",
        "FEAT_ITE_IMPLEMENTED",
        "FEAT_LSE128_IMPLEMENTED",
        "FEAT_LVA3_IMPLEMENTED",
        "FEAT_SEBEP_IMPLEMENTED",
        "FEAT_SME2p1_IMPLEMENTED",
        "FEAT_SME_F16F16_IMPLEMENTED",
        "FEAT_SVE2p1_IMPLEMENTED",
        "FEAT_SVE_B16B16_IMPLEMENTED",
        "FEAT_SYSINSTR128_IMPLEMENTED",
        "FEAT_SYSREG128_IMPLEMENTED",
        "FEAT_TRBE_EXT_IMPLEMENTED",
        "FEAT_TRBE_MPAM_IMPLEMENTED",
        "v8Ap0_IMPLEMENTED",
        "v8Ap1_IMPLEMENTED",
        "v8Ap2_IMPLEMENTED",
        "v8Ap3_IMPLEMENTED",
        "v8Ap4_IMPLEMENTED",
        "v8Ap5_IMPLEMENTED",
        "v8Ap6_IMPLEMENTED",
        "v8Ap7_IMPLEMENTED",
        "v8Ap8_IMPLEMENTED",
        "v8Ap9_IMPLEMENTED",
        "v9Ap0_IMPLEMENTED",
        "v9Ap1_IMPLEMENTED",
        "v9Ap2_IMPLEMENTED",
        "v9Ap3_IMPLEMENTED",
        "v9Ap4_IMPLEMENTED",
    ];

    let enabled = [
        "FEAT_AA64EL0_IMPLEMENTED",
        "FEAT_AA64EL1_IMPLEMENTED",
        "FEAT_AA64EL2_IMPLEMENTED",
        "FEAT_AA64EL3_IMPLEMENTED",
        "FEAT_D128_IMPLEMENTED",
        "FEAT_LVA3_IMPLEMENTED",
    ];

    features
        .iter()
        .map(|name| (name, enabled.contains(name)))
        .for_each(|(name, value)| {
            let offset = model.reg_offset(name);
            unsafe { register_file.add(offset as usize).write(value as u8) };
        });
}
