//! BOREALIS GENERATED FILE DO NOT MODIFY
use super::{CoreState, ExecutionEngine};
pub struct AArch64Interpreter;
pub struct AArch64CoreState {
    pc: u64,
    sp: u64,
    x: [u64; 31],
}
impl CoreState for AArch64CoreState {
    fn pc(&self) -> usize {
        self.pc as usize
    }
    fn new(pc: usize) -> Self {
        Self {
            pc: pc as u64,
            sp: 0,
            x: [0; 31],
        }
    }
}
impl ExecutionEngine<AArch64CoreState> for AArch64Interpreter {
    fn step(amount: super::StepAmount, state: &mut AArch64CoreState) -> super::StepResult {
        let insn_data = fetch(state.pc());
        log::trace!("fetch @ {} = {:08x}", state.pc(), insn_data);
        match decode_execute(insn_data, state) {
            ExecuteResult::Ok => {
                state.pc += 4;
                super::StepResult::Ok
            }
            ExecuteResult::EndOfBlock => super::StepResult::Ok,
            ExecuteResult::UndefinedInstruction => {
                panic!("undefined instruction {:08x}", insn_data)
            }
        }
    }
}
fn fetch(pc: usize) -> u32 {
    unsafe { *(pc as *const u32) }
}
enum ExecuteResult {
    Ok,
    EndOfBlock,
    UndefinedInstruction,
}
fn decode_execute(value: u32, state: &mut AArch64CoreState) -> ExecuteResult {
    if (value & 0x0000001f) == 0x0000001f {
        if (value & 0x000001c0) == 0x00000100 {
            if (value & 0xfffffc00) == 0xd5032000 {
                return integer_pac_pacia_hint_decode(state);
            }
        } else if (value & 0x000001c0) == 0x00000140 {
            if (value & 0xfffffc00) == 0xd5032000 {
                return integer_pac_pacib_hint_decode(state);
            }
        } else if (value & 0x000001c0) == 0x000001c0 {
            if (value & 0xfffffc00) == 0xd5032000 {
                return integer_pac_autib_hint_decode(state);
            }
        } else if (value & 0x000001c0) == 0x00000180 {
            if (value & 0xfffffc00) == 0xd5032000 {
                return integer_pac_autia_hint_decode(state);
            }
        }
        if (value & 0x0000fc00) == 0x00007000 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0x3f800000) == 0x38000000 {
                    if (value & 0x80000000) == 0x80000000 {
                        return memory_atomicops_st_decode(state);
                    }
                } else if (value & 0xff800000) == 0x38000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0xff800000) == 0x78000000 {
                    return memory_atomicops_st_decode(state);
                }
            }
        } else if (value & 0x0000fc00) == 0x00004000 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0x3f800000) == 0x38000000 {
                    if (value & 0x80000000) == 0x80000000 {
                        return memory_atomicops_st_decode(state);
                    }
                } else if (value & 0xff800000) == 0x78000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0xff800000) == 0x38000000 {
                    return memory_atomicops_st_decode(state);
                }
            }
        } else if (value & 0x0000fc00) == 0x00003000 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0x3f800000) == 0x38000000 {
                    if (value & 0x80000000) == 0x80000000 {
                        return memory_atomicops_st_decode(state);
                    }
                } else if (value & 0xff800000) == 0x78000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0xff800000) == 0x38000000 {
                    return memory_atomicops_st_decode(state);
                }
            }
        } else if (value & 0x0000fc00) == 0x00006000 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0xff800000) == 0x38000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0xff800000) == 0x78000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0x3f800000) == 0x38000000 {
                    if (value & 0x80000000) == 0x80000000 {
                        return memory_atomicops_st_decode(state);
                    }
                }
            }
        } else if (value & 0x0000fc00) == 0x00002000 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0xff800000) == 0x38000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0x3f800000) == 0x38000000 {
                    if (value & 0x80000000) == 0x80000000 {
                        return memory_atomicops_st_decode(state);
                    }
                } else if (value & 0xff800000) == 0x78000000 {
                    return memory_atomicops_st_decode(state);
                }
            }
        } else if (value & 0x0000fc00) == 0x00001000 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0xff800000) == 0x78000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0x3f800000) == 0x38000000 {
                    if (value & 0x80000000) == 0x80000000 {
                        return memory_atomicops_st_decode(state);
                    }
                } else if (value & 0xff800000) == 0x38000000 {
                    return memory_atomicops_st_decode(state);
                }
            }
        } else if (value & 0x0000fc00) == 0x00000000 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0xff800000) == 0x78000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0x3f800000) == 0x38000000 {
                    if (value & 0x80000000) == 0x80000000 {
                        return memory_atomicops_st_decode(state);
                    }
                } else if (value & 0xff800000) == 0x38000000 {
                    return memory_atomicops_st_decode(state);
                }
            }
        } else if (value & 0x0000fc00) == 0x00005000 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0x3f800000) == 0x38000000 {
                    if (value & 0x80000000) == 0x80000000 {
                        return memory_atomicops_st_decode(state);
                    }
                } else if (value & 0xff800000) == 0x38000000 {
                    return memory_atomicops_st_decode(state);
                } else if (value & 0xff800000) == 0x78000000 {
                    return memory_atomicops_st_decode(state);
                }
            }
        }
        if (value & 0xfffff000) == 0xd5032000 {
            return system_hints_decode(state);
        } else if (value & 0x0000f000) == 0x00004000 {
            if (value & 0xfff80000) == 0xd5000000 {
                return system_register_cpsr_decode(state);
            }
        }
    } else if (value & 0x0000001f) == 0x00000002 {
        if (value & 0xffe00000) == 0xd4000000 {
            return system_exceptions_runtime_hvc_decode(state);
        } else if (value & 0xffe00000) == 0xd4a00000 {
            return system_exceptions_debug_exception_decode(state);
        }
    }
    if value == 0xd503203f {
        return system_hints_decode(state);
    } else if (value & 0x0000001f) == 0x00000000 {
        if (value & 0xfffffc00) == 0xd61f0000 {
            return branch_unconditional_register_decode(state);
        } else if (value & 0xfffffc00) == 0xd63f0000 {
            return branch_unconditional_register_decode(state);
        } else if (value & 0xfffffc00) == 0xd65f0000 {
            return branch_unconditional_register_decode(state);
        }
        if (value & 0xffe00000) == 0xd4400000 {
            return system_exceptions_debug_halt_decode(state);
        } else if (value & 0xffe00000) == 0xd4200000 {
            return system_exceptions_debug_breakpoint_decode(state);
        }
    } else if (value & 0x0000001f) == 0x0000000d {
        if (value & 0x00003c00) == 0x00000800 {
            if (value & 0x7fff8000) == 0x3a000000 {
                return integer_flags_setf_decode(state);
            }
        }
    }
    if value == 0xd503205f {
        return system_hints_decode(state);
    }
    if value == 0xd6bf03e0 {
        return branch_unconditional_dret_decode(state);
    } else if (value & 0x000000ff) == 0x0000009f {
        if (value & 0xfffff000) == 0xd5033000 {
            return system_barriers_decode(state);
        }
    } else if (value & 0x00000007) == 0x00000000 {
        if (value & 0x00000010) == 0x00000010 {
            if (value & 0x0000fc00) == 0x00002000 {
                if (value & 0x00200000) == 0x00200000 {
                    if (value & 0xff000000) == 0x1e000000 {
                        return float_compare_uncond_decode(state);
                    }
                }
            }
        } else if (value & 0x00000010) == 0x00000000 {
            if (value & 0x0000fc00) == 0x00002000 {
                if (value & 0x00200000) == 0x00200000 {
                    if (value & 0xff000000) == 0x1e000000 {
                        return float_compare_uncond_decode(state);
                    }
                }
            }
        }
    } else if (value & 0x0000001f) == 0x00000003 {
        if (value & 0xffe00000) == 0xd4a00000 {
            return system_exceptions_debug_exception_decode(state);
        } else if (value & 0xffe00000) == 0xd4000000 {
            return system_exceptions_runtime_smc_decode(state);
        }
    } else if (value & 0x000000ff) == 0x000000ff {
        if (value & 0xfffff000) == 0xd5033000 {
            return system_barriers_decode(state);
        }
    }
    if value == 0xd503223f {
        return system_hints_decode(state);
    } else if (value & 0x000000ff) == 0x0000005f {
        if (value & 0xfffff000) == 0xd5033000 {
            return system_monitors_decode(state);
        } else if (value & 0xfffff000) == 0xd5004000 {
            return integer_flags_axflag_decode(state);
        }
    }
    if value == 0xd69f03e0 {
        return branch_unconditional_eret_decode(state);
    } else if (value & 0x000003ff) == 0x000003ff {
        if (value & 0xfffff800) == 0xd69f0800 {
            return branch_unconditional_eret_decode(state);
        } else if (value & 0xfffff800) == 0xd65f0800 {
            return branch_unconditional_register_decode(state);
        }
    }
    if value == 0xd50320bf {
        return system_hints_decode(state);
    }
    if value == 0xd500401f {
        return integer_flags_cfinv_decode(state);
    } else if (value & 0x000000ff) == 0x000000df {
        if (value & 0xfffff000) == 0xd5033000 {
            return system_barriers_decode(state);
        }
    }
    if value == 0xd503207f {
        return system_hints_decode(state);
    }
    if value == 0xd503209f {
        return system_hints_decode(state);
    }
    if value == 0xd503221f {
        return system_hints_decode(state);
    } else if (value & 0x000000ff) == 0x0000003f {
        if (value & 0xfffff000) == 0xd5004000 {
            return integer_flags_xaflag_decode(state);
        }
    }
    if value == 0xd503201f {
        return system_hints_decode(state);
    } else if (value & 0x0000001f) == 0x00000001 {
        if (value & 0xffe00000) == 0xd4a00000 {
            return system_exceptions_debug_exception_decode(state);
        } else if (value & 0xffe00000) == 0xd4000000 {
            return system_exceptions_runtime_svc_decode(state);
        }
    }
    if value == 0xd50320ff {
        return integer_pac_strip_hint_decode(state);
    } else if (value & 0x000000ff) == 0x000000bf {
        if (value & 0xfffff000) == 0xd5033000 {
            return system_barriers_decode(state);
        }
    }
    if (value & 0x00000010) == 0x00000000 {
        if (value & 0x00000c00) == 0x00000400 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0xff000000) == 0x1e000000 {
                    return float_compare_cond_decode(state);
                }
            }
        } else if (value & 0x00007c00) == 0x00000400 {
            if (value & 0x7fe00000) == 0x3a000000 {
                return integer_flags_rmif_decode(state);
            }
        } else if (value & 0x00000c00) == 0x00000800 {
            if (value & 0x7fe00000) == 0x7a400000 {
                return integer_conditional_compare_immediate_decode(state);
            } else if (value & 0x7fe00000) == 0x3a400000 {
                return integer_conditional_compare_immediate_decode(state);
            }
        } else if (value & 0x00000c00) == 0x00000000 {
            if (value & 0x7fe00000) == 0x7a400000 {
                return integer_conditional_compare_register_decode(state);
            } else if (value & 0x7fe00000) == 0x3a400000 {
                return integer_conditional_compare_register_decode(state);
            }
        }
        if (value & 0xff000000) == 0x54000000 {
            return branch_conditional_cond_decode(state);
        }
    } else if (value & 0x00000010) == 0x00000010 {
        if (value & 0x00000c00) == 0x00000400 {
            if (value & 0x00200000) == 0x00200000 {
                if (value & 0xff000000) == 0x1e000000 {
                    return float_compare_cond_decode(state);
                }
            }
        }
    }
    if (value & 0x00001fe0) == 0x00001000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_move_fp_imm_decode(state);
            }
        }
    } else if (value & 0x000003e0) == 0x000003e0 {
        if (value & 0xfffff800) == 0xdac14000 {
            return integer_pac_strip_dp_1src_decode(state);
        }
    }
    if (value & 0x00000c00) == 0x00000800 {
        if (value & 0x003fe000) == 0x0021e000 {
            if (value & 0x1f800000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_unary_float_round_frint_32_64_decode(state);
                }
            }
        }
        if (value & 0x3fe00000) == 0x38000000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                    state,
                );
            }
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0xff800000) == 0x78800000 {
                return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                    state,
                );
            } else if (value & 0xff800000) == 0x38800000 {
                return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x38200000 {
            return memory_single_general_register_memory_single_general_register__decode(state);
        } else if (value & 0xffe00000) == 0x19a00000 {
            return integer_tags_mcsettagpairandzerodata_decode(state);
        } else if (value & 0xffe00000) == 0x78000000 {
            return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0xb8a00000 {
            return memory_single_general_register_memory_single_general_register__decode(state);
        } else if (value & 0xffe00000) == 0x78600000 {
            return memory_single_general_register_memory_single_general_register__decode(state);
        } else if (value & 0xffe00000) == 0x38600000 {
            return memory_single_general_register_memory_single_general_register__decode(state);
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff800000) == 0x38800000 {
                return memory_single_general_register_memory_single_general_register__decode(
                    state,
                );
            } else if (value & 0xff800000) == 0x78800000 {
                return memory_single_general_register_memory_single_general_register__decode(
                    state,
                );
            }
        } else if (value & 0x00600000) == 0x00600000 {
            if (value & 0x3f000000) == 0x3c000000 {
                return memory_single_simdfp_register_memory_single_simdfp_register__decode(state);
            }
        } else if (value & 0xffe00000) == 0x19000000 {
            return integer_tags_mcsettag_decode(state);
        } else if (value & 0x3fe00000) == 0x38200000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_register_memory_single_general_register__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x78400000 {
            return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0xf8a00000 {
            return memory_single_general_register_memory_single_general_register__decode(state);
        } else if (value & 0xffe00000) == 0x19800000 {
            return integer_tags_mcsettagandzerodata_decode(state);
        } else if (value & 0x00600000) == 0x00200000 {
            if (value & 0x3f000000) == 0x3c000000 {
                return memory_single_simdfp_register_memory_single_simdfp_register__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x38400000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                    state,
                );
            }
        } else if (value & 0x3fe00000) == 0x38600000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_register_memory_single_general_register__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0xb8800000 {
            return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x38400000 {
            return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x19400000 {
            return integer_tags_mcgettag_decode(state);
        } else if (value & 0xffe00000) == 0x38000000 {
            return memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x19200000 {
            return integer_tags_mcsettagpair_decode(state);
        } else if (value & 0xffe00000) == 0x78200000 {
            return memory_single_general_register_memory_single_general_register__decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00004000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_ld_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_narrow_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_narrow_decode(state);
                }
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_ld_decode(state);
            }
        } else if (value & 0xffe00000) == 0x5e000000 {
            return vector_crypto_sha3op_sha256hash_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x5e282800 {
        return vector_crypto_sha2op_sha256sched0_decode(state);
    } else if (value & 0x0000fc00) == 0x00000400 {
        if (value & 0x3fe00000) == 0x2e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode(state);
            }
        } else if (value & 0xffe00000) == 0x5e000000 {
            return vector_transfer_vector_cpydup_sisd_decode(state);
        } else if (value & 0x3fe00000) == 0x0e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x2ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode(state);
            }
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_halving_truncating_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_halving_truncating_decode(state);
                }
            }
        } else if (value & 0x3fe00000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_transfer_vector_cpydup_simd_decode(state);
            }
        }
        if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_right_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_right_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_right_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_right_simd_decode(state);
            }
        }
    } else if (value & 0xfffffc00) == 0x5e30d800 {
        return vector_reduce_fp16add_sisd_decode(state);
    } else if (value & 0x00000c00) == 0x00000000 {
        if (value & 0xffe00000) == 0x59c00000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x99800000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x19000000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x59000000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x38000000 {
            return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x78000000 {
            return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0xb8800000 {
            return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0xd9000000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x19400000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0x7fe00000) == 0x1a800000 {
            return integer_conditional_select_decode(state);
        } else if (value & 0xffe00000) == 0x19800000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0xf8800000 {
            return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                state,
            );
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0xff800000) == 0x38800000 {
                return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                    state,
                );
            } else if (value & 0xff800000) == 0x78800000 {
                return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x19c00000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x78400000 {
            return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x59400000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x59800000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0x00600000) == 0x00400000 {
            if (value & 0x3f000000) == 0x3c000000 {
                return memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x99400000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0xd9400000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0x3fe00000) == 0x38000000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                    state,
                );
            }
        } else if (value & 0x00600000) == 0x00000000 {
            if (value & 0x3f000000) == 0x3c000000 {
                return memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x99000000 {
            return memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x38400000 {
            return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                state,
            );
        } else if (value & 0x3fe00000) == 0x38400000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
                    state,
                );
            }
        } else if (value & 0x7fe00000) == 0x5a800000 {
            return integer_conditional_select_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00005c00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_shift_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_shift_sisd_decode(state);
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_shift_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_shift_simd_decode(state);
                }
            }
        }
    } else if (value & 0x0000fc00) == 0x00000000 {
        if (value & 0x00060000) == 0x00060000 {
            if (value & 0x00300000) == 0x00200000 {
                if (value & 0x7f000000) == 0x1e000000 {
                    return float_convert_int_decode(state);
                }
            }
        }
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_long_decode(state);
                }
            } else if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_long_decode(state);
                }
            } else if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_ld_decode(state);
                }
            }
        } else if (value & 0x7fe00000) == 0x3a000000 {
            return integer_arithmetic_addsub_carry_decode(state);
        } else if (value & 0xffe00000) == 0xbac00000 {
            return integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode(state);
        } else if (value & 0xffe00000) == 0x9ac00000 {
            return integer_arithmetic_pointer_mcsubtracttaggedaddress_decode(state);
        } else if (value & 0x7fe00000) == 0x5a000000 {
            return integer_arithmetic_addsub_carry_decode(state);
        } else if (value & 0xffe00000) == 0x5e000000 {
            return vector_crypto_sha3op_sha1hash_choose_decode(state);
        } else if (value & 0x7fe00000) == 0x7a000000 {
            return integer_arithmetic_addsub_carry_decode(state);
        } else if (value & 0x7fe00000) == 0x1a000000 {
            return integer_arithmetic_addsub_carry_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0e79b800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00209800 {
        if (value & 0xff000000) == 0x7e000000 {
            return vector_arithmetic_unary_cmp_int_bulk_sisd_decode(state);
        } else if (value & 0xff000000) == 0x5e000000 {
            return vector_arithmetic_unary_cmp_int_bulk_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_int_bulk_simd_decode(state);
            }
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_int_bulk_simd_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x00201800 {
        if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_rev_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x00002400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_sub_int_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_sub_int_decode(state);
                }
            }
        } else if (value & 0x7fe00000) == 0x1ac00000 {
            return integer_shift_variable_decode(state);
        } else if (value & 0x3fe00000) == 0x2e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_cmp_fp16_simd_decode(state);
            }
        } else if (value & 0xffe00000) == 0x7e400000 {
            return vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode(state);
        } else if (value & 0x3fe00000) == 0x2ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_cmp_fp16_simd_decode(state);
            }
        } else if (value & 0xffe00000) == 0x5e400000 {
            return vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode(state);
        } else if (value & 0xffe00000) == 0x7ec00000 {
            return vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode(state);
        } else if (value & 0x3fe00000) == 0x0e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_cmp_fp16_simd_decode(state);
            }
        }
        if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_right_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_right_sisd_decode(state);
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_right_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_right_simd_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x00207800 {
        if (value & 0xff000000) == 0x5e000000 {
            return vector_arithmetic_unary_diffneg_sat_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_diffneg_sat_simd_decode(state);
            }
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_diffneg_sat_simd_decode(state);
            }
        } else if (value & 0xff000000) == 0x7e000000 {
            return vector_arithmetic_unary_diffneg_sat_sisd_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x4e287800 {
        return vector_crypto_aes_mix_decode(state);
    } else if (value & 0x003ffc00) == 0x0021c800 {
        if (value & 0x3f800000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_tieaway_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_tieaway_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_special_sqrtest_int_decode(state);
            }
        } else if (value & 0xff800000) == 0x5e000000 {
            return vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_special_recip_int_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e000000 {
            return vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000c000 {
        if (value & 0xffe00000) == 0x38a00000 {
            return memory_orderedrcpc_decode(state);
        } else if (value & 0xffe00000) == 0x78a00000 {
            return memory_orderedrcpc_decode(state);
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_product_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_product_decode(state);
                }
            }
        } else if (value & 0x3fe00000) == 0x38a00000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_orderedrcpc_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x2ef8f800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_diffneg_fp16_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00007c00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_diff_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_diff_decode(state);
                }
            }
        }
    } else if (value & 0x00000400) == 0x00000000 {
        if (value & 0x0000f000) == 0x0000f000 {
            if (value & 0xff000000) == 0x7f000000 {
                return vector_arithmetic_binary_element_mulacc_high_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_high_simd_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x0000d000 {
            if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_high_simd_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_high_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5f000000 {
                return vector_arithmetic_binary_element_mul_high_sisd_decode(state);
            } else if (value & 0xff000000) == 0x7f000000 {
                return vector_arithmetic_binary_element_mulacc_high_sisd_decode(state);
            }
        } else if (value & 0x0000f000) == 0x00001000 {
            if (value & 0xffc00000) == 0x5f000000 {
                return vector_arithmetic_binary_element_mulacc_fp16_sisd_decode(state);
            } else if (value & 0x3fc00000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_fp16_simd_decode(state);
                }
            }
            if (value & 0xff800000) == 0x5f800000 {
                return vector_arithmetic_binary_element_mulacc_fp_sisd_decode(state);
            } else if (value & 0x3f800000) == 0x0f800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_fp_simd_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x0000c000 {
            if (value & 0xff000000) == 0x5f000000 {
                return vector_arithmetic_binary_element_mul_high_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_high_simd_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x0000b000 {
            if (value & 0xff000000) == 0x5f000000 {
                return vector_arithmetic_binary_element_mul_double_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_double_simd_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x00005000 {
            if (value & 0xffc00000) == 0x5f000000 {
                return vector_arithmetic_binary_element_mulacc_fp16_sisd_decode(state);
            } else if (value & 0x3fc00000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_fp16_simd_decode(state);
                }
            }
            if (value & 0xff800000) == 0x5f800000 {
                return vector_arithmetic_binary_element_mulacc_fp_sisd_decode(state);
            } else if (value & 0x3f800000) == 0x0f800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_fp_simd_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x00004000 {
            if (value & 0x3f000000) == 0x2f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_int_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x00009000 {
            if (value & 0x3fc00000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_fp16_simd_decode(state);
                }
            } else if (value & 0x3fc00000) == 0x2f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_fp16_simd_decode(state);
                }
            } else if (value & 0xffc00000) == 0x7f000000 {
                return vector_arithmetic_binary_element_mul_fp16_sisd_decode(state);
            } else if (value & 0xffc00000) == 0x5f000000 {
                return vector_arithmetic_binary_element_mul_fp16_sisd_decode(state);
            }
            if (value & 0x3f800000) == 0x2f800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_fp_simd_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0f800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_fp_simd_decode(state);
                }
            } else if (value & 0xff800000) == 0x5f800000 {
                return vector_arithmetic_binary_element_mul_fp_sisd_decode(state);
            } else if (value & 0xff800000) == 0x7f800000 {
                return vector_arithmetic_binary_element_mul_fp_sisd_decode(state);
            }
        } else if (value & 0x0000f000) == 0x00007000 {
            if (value & 0xff000000) == 0x5f000000 {
                return vector_arithmetic_binary_element_mulacc_double_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_double_simd_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x0000a000 {
            if (value & 0x3f000000) == 0x2f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_long_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_long_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x00002000 {
            if (value & 0x3f000000) == 0x2f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_long_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_long_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x00006000 {
            if (value & 0x3f000000) == 0x2f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_long_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_long_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x00008000 {
            if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mul_int_decode(state);
                }
            }
        } else if (value & 0x00001000) == 0x00001000 {
            if (value & 0x00008000) == 0x00000000 {
                if (value & 0x3f000000) == 0x2f000000 {
                    if (value & 0x80000000) == 0x00000000 {
                        return vector_arithmetic_binary_element_mulacc_complex_decode(state);
                    }
                }
            }
        } else if (value & 0x0000f000) == 0x00000000 {
            if (value & 0x3f000000) == 0x2f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_int_decode(state);
                }
            }
        } else if (value & 0x0000f000) == 0x00003000 {
            if (value & 0x3f000000) == 0x0f000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_element_mulacc_double_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5f000000 {
                return vector_arithmetic_binary_element_mulacc_double_sisd_decode(state);
            }
        } else if (value & 0x00003000) == 0x00000000 {
            if (value & 0x00008000) == 0x00008000 {
                if (value & 0x3f800000) == 0x2f800000 {
                    if (value & 0x80000000) == 0x00000000 {
                        return vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode(
                            state,
                        );
                    }
                }
            } else if (value & 0x00008000) == 0x00000000 {
                if (value & 0x3f800000) == 0x0f800000 {
                    if (value & 0x80000000) == 0x00000000 {
                        return vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode(
                            state,
                        );
                    }
                }
            }
        }
        if (value & 0x00008000) == 0x00000000 {
            if (value & 0x3fe00000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_extract_decode(state);
                }
            }
        }
    } else if (value & 0x00000c00) == 0x00000400 {
        if (value & 0x0000e000) == 0x0000e000 {
            if (value & 0x00200000) == 0x00000000 {
                if (value & 0x3f000000) == 0x2e000000 {
                    if (value & 0x80000000) == 0x00000000 {
                        return vector_arithmetic_binary_uniform_add_fp_complex_decode(state);
                    }
                }
            }
        }
        if (value & 0x3ff80000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_logical_decode(state);
            }
        } else if (value & 0x1ff80000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_logical_decode(state);
            }
        }
        if (value & 0x3fe00000) == 0x38000000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x38400000 {
            return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x38000000 {
            return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        } else if (value & 0x00600000) == 0x00000000 {
            if (value & 0x3f000000) == 0x3c000000 {
                return memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0x00600000) == 0x00400000 {
            if (value & 0x3f000000) == 0x3c000000 {
                return memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0x3fe00000) == 0x38400000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0x7fe00000) == 0x5a800000 {
            return integer_conditional_select_decode(state);
        } else if (value & 0xffe00000) == 0x19a00000 {
            return integer_tags_mcsettagpairandzerodatapost_decode(state);
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0xff800000) == 0x78800000 {
                return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                    state,
                );
            } else if (value & 0xff800000) == 0x38800000 {
                return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x19000000 {
            return integer_tags_mcsettagpost_decode(state);
        } else if (value & 0xffe00000) == 0x19800000 {
            return integer_tags_mcsettagandzerodatapost_decode(state);
        } else if (value & 0xffe00000) == 0x78400000 {
            return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x19200000 {
            return integer_tags_mcsettagpairpost_decode(state);
        } else if (value & 0xffe00000) == 0xb8800000 {
            return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        } else if (value & 0x7fe00000) == 0x1a800000 {
            return integer_conditional_select_decode(state);
        } else if (value & 0xffe00000) == 0x78000000 {
            return memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        }
    } else if (value & 0x0000fc00) == 0x00001800 {
        if (value & 0x00200000) == 0x00000000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_permute_unzip_decode(state);
                }
            }
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_div_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x0000a000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_accum_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_accum_decode(state);
                }
            }
        }
    } else if (value & 0x003ffc00) == 0x0030f800 {
        if (value & 0xff800000) == 0x7e000000 {
            return vector_reduce_fpmax_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_fpmax_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_fpmax_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e800000 {
            return vector_reduce_fpmax_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00004c00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_shift_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_shift_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_shift_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_shift_sisd_decode(state);
            }
        }
    } else if (value & 0x00001c00) == 0x00001400 {
        if (value & 0xffffc000) == 0xdac10000 {
            return integer_pac_autib_dp_1src_decode(state);
        }
        if (value & 0x3ff80000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_logical_decode(state);
            }
        } else if (value & 0x3ff80000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_logical_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x00213800 {
        if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_shift_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x00003400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_int_simd_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_int_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_cmp_int_sisd_decode(state);
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_cmp_int_sisd_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x2ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x2e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode(state);
            }
        }
        if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_right_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_right_sisd_decode(state);
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_right_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_right_simd_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x00214800 {
        if (value & 0xff000000) == 0x7e000000 {
            return vector_arithmetic_unary_extract_sat_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_extract_sat_simd_decode(state);
            }
        } else if (value & 0xff000000) == 0x5e000000 {
            return vector_arithmetic_unary_extract_sat_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_extract_sat_simd_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x00003c00 {
        if (value & 0xffe00000) == 0x5e400000 {
            return vector_arithmetic_binary_uniform_recpsfp16_sisd_decode(state);
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_int_simd_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_int_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_cmp_int_sisd_decode(state);
            } else if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_cmp_int_sisd_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_recpsfp16_simd_decode(state);
            }
        } else if (value & 0xffe00000) == 0x5ec00000 {
            return vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode(state);
        } else if (value & 0x3fe00000) == 0x2e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_divfp16_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_transfer_integer_move_unsigned_decode(state);
            }
        }
    } else if (value & 0xfffffc00) == 0x5eb0c800 {
        return vector_reduce_fp16maxnm_sisd_decode(state);
    } else if (value & 0x003ffc00) == 0x00303800 {
        if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_addlong_decode(state);
            }
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_addlong_decode(state);
            }
        }
    } else if (value & 0xfffffc00) == 0xdac00800 {
        return integer_arithmetic_rev_decode(state);
    } else if (value & 0x0000fc00) == 0x00001400 {
        if (value & 0xffe00000) == 0x9ac00000 {
            return integer_tags_mcinserttagmask_decode(state);
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_halving_rounding_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_halving_rounding_decode(state);
                }
            }
        } else if (value & 0x3fe00000) == 0x2e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_add_fp16_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_add_fp16_decode(state);
            }
        } else if (value & 0xffe00000) == 0x7ec00000 {
            return vector_arithmetic_binary_uniform_sub_fp16_sisd_decode(state);
        } else if (value & 0x3fe00000) == 0x2ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_sub_fp16_simd_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_sub_fp16_simd_decode(state);
            }
        }
        if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_right_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_right_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_right_sisd_decode(state);
        } else if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_right_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00002000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_ld_decode(state);
                }
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_long_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_long_decode(state);
                }
            } else if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_ld_decode(state);
            }
        } else if (value & 0x7fe00000) == 0x1ac00000 {
            return integer_shift_variable_decode(state);
        } else if (value & 0xffe00000) == 0x5e000000 {
            return vector_crypto_sha3op_sha1hash_majority_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00300000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00006000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_narrow_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_narrow_decode(state);
                }
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_ld_decode(state);
                }
            }
        } else if (value & 0xffe00000) == 0x5e000000 {
            return vector_crypto_sha3op_sha256sched1_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00208800 {
        if (value & 0xff000000) == 0x7e000000 {
            return vector_arithmetic_unary_cmp_int_bulk_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_int_bulk_simd_decode(state);
            }
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_int_bulk_simd_decode(state);
            }
        } else if (value & 0xff000000) == 0x5e000000 {
            return vector_arithmetic_unary_cmp_int_bulk_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000f400 {
        if (value & 0x1ff80000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_logical_decode(state);
            }
        }
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f800000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_fp_1985_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_fp_1985_decode(state);
                }
            } else if (value & 0x3f800000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_fp_1985_decode(state);
                }
            } else if (value & 0x3f800000) == 0x2e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_fp_1985_decode(state);
                }
            }
        }
    } else if (value & 0x003ffc00) == 0x0030c800 {
        if (value & 0xff800000) == 0x7e000000 {
            return vector_reduce_fpmaxnm_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_fpmaxnm_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_fpmaxnm_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e800000 {
            return vector_reduce_fpmaxnm_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00001c00 {
        if (value & 0x3fe00000) == 0x0e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0ea00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_logical_andorr_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0e600000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_logical_andorr_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0ee00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_logical_andorr_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x2e600000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_logical_bsleor_decode(state);
            }
        } else if (value & 0xffe00000) == 0x5e400000 {
            return vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode(state);
        } else if (value & 0x3fe00000) == 0x2e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_mul_fp16_product_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0e200000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_logical_andorr_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x2e200000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_logical_bsleor_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x2ea00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_logical_bsleor_decode(state);
            }
        } else if (value & 0xffe00000) == 0x4e000000 {
            return vector_transfer_integer_insert_decode(state);
        } else if (value & 0x3fe00000) == 0x2ee00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_logical_bsleor_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x0ef8e800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x0021a800 {
        if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_bulk_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e000000 {
            return vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(state);
        } else if (value & 0xff800000) == 0x5e800000 {
            return vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_bulk_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_bulk_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5e000000 {
            return vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_bulk_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e800000 {
            return vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(state);
        }
    } else if (value & 0x00000c00) == 0x00000c00 {
        if (value & 0x00200000) == 0x00000000 {
            if (value & 0xff800000) == 0x38800000 {
                return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                    state,
                );
            } else if (value & 0xff800000) == 0x78800000 {
                return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x38400000 {
            return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x19800000 {
            return integer_tags_mcsettagandzerodatapre_decode(state);
        } else if (value & 0x00600000) == 0x00400000 {
            if (value & 0x3f000000) == 0x3c000000 {
                return memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x78000000 {
            return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0x19a00000 {
            return integer_tags_mcsettagpairandzerodatapre_decode(state);
        } else if (value & 0xffe00000) == 0x19000000 {
            return integer_tags_mcsettagpre_decode(state);
        } else if (value & 0xffe00000) == 0x38000000 {
            return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        } else if (value & 0xffe00000) == 0xb8800000 {
            return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        } else if (value & 0x00600000) == 0x00000000 {
            if (value & 0x3f000000) == 0x3c000000 {
                return memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0x3fe00000) == 0x38000000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0x3fe00000) == 0x38400000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                    state,
                );
            }
        } else if (value & 0xffe00000) == 0x19200000 {
            return integer_tags_mcsettagpairpre_decode(state);
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_move_fp_select_decode(state);
            }
        } else if (value & 0xffe00000) == 0x78400000 {
            return memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        }
    } else if (value & 0x0000fc00) == 0x0000e400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff800000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_cmp_fp_sisd_decode(state);
            } else if (value & 0x3f800000) == 0x2e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_fp_simd_decode(state);
                }
            } else if (value & 0xff800000) == 0x7e800000 {
                return vector_arithmetic_binary_uniform_cmp_fp_sisd_decode(state);
            } else if (value & 0x3f800000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_fp_simd_decode(state);
                }
            } else if (value & 0x3f800000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_fp_simd_decode(state);
                }
            } else if (value & 0xff800000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_cmp_fp_sisd_decode(state);
            }
        }
        if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_conv_int_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_conv_int_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_conv_int_sisd_decode(state);
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_conv_int_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00009c00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_int_product_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_int_product_decode(state);
                }
            }
        }
        if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightnarrow_uniform_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_rightnarrow_uniform_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightnarrow_uniform_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_rightnarrow_uniform_sisd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x0021f800 {
        if (value & 0xff800000) == 0x5e800000 {
            return vector_arithmetic_unary_special_frecpx_decode(state);
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_special_sqrt_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x00204800 {
        if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_clsz_decode(state);
            }
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_clsz_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x0020d800 {
        if (value & 0xff800000) == 0x7e800000 {
            return vector_arithmetic_unary_cmp_float_bulk_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_float_bulk_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_float_bulk_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5e800000 {
            return vector_arithmetic_unary_cmp_float_bulk_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00001000 {
        if (value & 0xffe00000) == 0x5e000000 {
            return vector_crypto_sha3op_sha1hash_parity_decode(state);
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_wide_decode(state);
                }
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_wide_decode(state);
                }
            } else if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_ld_decode(state);
                }
            }
        } else if (value & 0xffe00000) == 0x9ac00000 {
            return integer_tags_mcinsertrandomtag_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00000c00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_add_saturating_sisd_decode(state);
            } else if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_add_saturating_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_saturating_simd_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_saturating_simd_decode(state);
                }
            }
        } else if (value & 0x3fe00000) == 0x0ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_mul_fp16_fused_decode(state);
            }
        } else if (value & 0x7fe00000) == 0x1ac00000 {
            return integer_arithmetic_div_decode(state);
        } else if (value & 0x3fe00000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_transfer_integer_dup_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_mul_fp16_fused_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x0000fc00 {
        if (value & 0x3ff80000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_fp16_movi_decode(state);
            }
        }
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f800000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_recps_simd_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_rsqrts_simd_decode(state);
                }
            } else if (value & 0x3f800000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_div_decode(state);
                }
            } else if (value & 0xff800000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_recps_sisd_decode(state);
            } else if (value & 0xff800000) == 0x5e800000 {
                return vector_arithmetic_binary_uniform_rsqrts_sisd_decode(state);
            }
        }
        if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_conv_float_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_conv_float_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_conv_float_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_conv_float_sisd_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x2ef8d800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_cmp_fp16_bulk_simd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00002c00 {
        if (value & 0x3fe00000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_transfer_integer_move_signed_decode(state);
            }
        } else if (value & 0xffe00000) == 0x7ec00000 {
            return vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode(state);
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_sub_saturating_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_sub_saturating_sisd_decode(state);
            } else if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_sub_saturating_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_sub_saturating_simd_decode(state);
                }
            }
        } else if (value & 0xffe00000) == 0x7e400000 {
            return vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode(state);
        } else if (value & 0x3fe00000) == 0x2e400000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_cmp_fp16_simd_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x2ec00000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_binary_uniform_cmp_fp16_simd_decode(state);
            }
        } else if (value & 0x7fe00000) == 0x1ac00000 {
            return integer_shift_variable_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00217800 {
        if (value & 0x3f800000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_widen_decode(state);
            }
        }
    } else if (value & 0x00007c00) == 0x00007c00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f800000) == 0x08800000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_cas_single_decode(state);
                }
            } else if (value & 0xff800000) == 0x48800000 {
                return memory_atomicops_cas_single_decode(state);
            } else if (value & 0xff800000) == 0x08800000 {
                return memory_atomicops_cas_single_decode(state);
            } else if (value & 0x3f800000) == 0x08000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return memory_atomicops_cas_pair_decode(state);
                }
            }
        }
    } else if (value & 0x3ffffc00) == 0x2ef8c800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_cmp_fp16_bulk_simd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00005400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_shift_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_shift_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_shift_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_shift_simd_decode(state);
                }
            }
        }
        if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_left_sisd_decode(state);
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_leftinsert_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_left_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_leftinsert_simd_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x0030a800 {
        if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_intmax_decode(state);
            }
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_intmax_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x00008000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_swp_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_accum_decode(state);
                }
            } else if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_swp_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_accum_decode(state);
                }
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_swp_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x0021d800 {
        if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_special_recip_float_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_int_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e000000 {
            return vector_arithmetic_unary_float_conv_int_sisd_decode(state);
        } else if (value & 0xff800000) == 0x5e800000 {
            return vector_arithmetic_unary_special_recip_float_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_special_sqrtest_float_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_int_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e800000 {
            return vector_arithmetic_unary_special_sqrtest_float_sisd_decode(state);
        } else if (value & 0xff800000) == 0x5e000000 {
            return vector_arithmetic_unary_float_conv_int_sisd_decode(state);
        }
    } else if (value & 0x00001c00) == 0x00000c00 {
        if (value & 0xffffc000) == 0xdac10000 {
            return integer_pac_pacdb_dp_1src_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00219800 {
        if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_round_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_round_decode(state);
            }
        } else if (value & 0x3f800000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_round_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_round_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x00202800 {
        if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_add_pairwise_decode(state);
            }
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_add_pairwise_decode(state);
            }
        }
    } else if (value & 0xfffffc00) == 0x7ef9b800 {
        return vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(state);
    } else if (value & 0x3ffffc00) == 0x0ef9a800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0ef8d800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_cmp_fp16_bulk_simd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00008400 {
        if (value & 0x00200000) == 0x00000000 {
            if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode(
                        state,
                    );
                }
            }
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode(state);
                }
            }
        }
        if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_rightnarrow_nonuniform_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightnarrow_logical_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightnarrow_nonuniform_simd_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x0024c000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_round_frint_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00007400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_diff_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_diff_decode(state);
                }
            }
        }
        if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_leftsat_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_leftsat_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_leftsat_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_leftsat_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000cc00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f800000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_fp_fused_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_fp_fused_decode(state);
                }
            }
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode(
                        state,
                    );
                }
            }
        }
    } else if (value & 0xfffffc00) == 0x5e79a800 {
        return vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(state);
    } else if (value & 0x0000fc00) == 0x00008c00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode(state);
            } else if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode(state);
                }
            }
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode(
                        state,
                    );
                }
            }
        }
        if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightnarrow_nonuniform_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightnarrow_logical_decode(state);
            }
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_rightnarrow_nonuniform_sisd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00250000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000b000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_dmacc_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode(state);
            }
        }
    } else if (value & 0xfffffc00) == 0x5ef9d800 {
        return vector_arithmetic_unary_special_recip_fp16_sisd_decode(state);
    } else if (value & 0xfffffc00) == 0x4e284800 {
        return vector_crypto_aes_round_decode(state);
    } else if (value & 0x0000fc00) == 0x0000ec00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f800000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_fp_simd_decode(state);
                }
            } else if (value & 0xff800000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_cmp_fp_sisd_decode(state);
            } else if (value & 0x3f800000) == 0x2e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_cmp_fp_simd_decode(state);
                }
            } else if (value & 0xff800000) == 0x7e800000 {
                return vector_arithmetic_binary_uniform_cmp_fp_sisd_decode(state);
            }
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode(
                        state,
                    );
                }
            }
        }
    } else if (value & 0x3ffffc00) == 0x2ef9a800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(state);
        }
    } else if (value & 0x00001c00) == 0x00001c00 {
        if (value & 0xffffc000) == 0xdac10000 {
            return integer_pac_autdb_dp_1src_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x0020c000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_unary_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00003000 {
        if (value & 0xffe00000) == 0x9ac00000 {
            return integer_pac_pacga_dp_2src_decode(state);
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_wide_decode(state);
                }
            } else if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_ld_decode(state);
                }
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_addsub_wide_decode(state);
                }
            }
        } else if (value & 0xffe00000) == 0x5e000000 {
            return vector_crypto_sha3op_sha1sched0_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x0020a800 {
        if (value & 0xff000000) == 0x5e000000 {
            return vector_arithmetic_unary_cmp_int_lessthan_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_int_lessthan_simd_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x00005800 {
        if (value & 0x00200000) == 0x00000000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_permute_unzip_decode(state);
                }
            }
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_maxmin_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x2e79d800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_int_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00212800 {
        if (value & 0xff000000) == 0x7e000000 {
            return vector_arithmetic_unary_extract_sqxtun_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_extract_sqxtun_simd_decode(state);
            }
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_extract_nosat_decode(state);
            }
        }
    } else if (value & 0xfffffc00) == 0x5ef8d800 {
        return vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode(state);
    } else if (value & 0x3ffffc00) == 0x0ef98800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_round_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00230000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x2e79c800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x0031a800 {
        if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_intmax_decode(state);
            }
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_intmax_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x00004400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_shift_sisd_decode(state);
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_shift_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_shift_simd_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_shift_simd_decode(state);
                }
            }
        }
        if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightinsert_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_rightinsert_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00009400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_int_accum_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_int_accum_decode(state);
                }
            }
        }
        if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightnarrow_uniform_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_rightnarrow_uniform_sisd_decode(state);
        } else if (value & 0xff800000) == 0x5f000000 {
            return vector_shift_rightnarrow_uniform_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_rightnarrow_uniform_simd_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x0000b400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x7e000000 {
                return vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode(state);
                }
            }
        }
    } else if (value & 0x003ffc00) == 0x00264000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_round_frint_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x7ef8d800 {
        return vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode(state);
    } else if (value & 0xfffffc00) == 0x5eb0f800 {
        return vector_reduce_fp16max_sisd_decode(state);
    } else if (value & 0x0000fc00) == 0x00007800 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_maxmin_decode(state);
            }
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_permute_zip_decode(state);
                }
            }
        }
    } else if (value & 0x0000fc00) == 0x00002800 {
        if (value & 0x7fe00000) == 0x1ac00000 {
            return integer_shift_variable_decode(state);
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_permute_transpose_decode(state);
                }
            }
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_addsub_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x0021b800 {
        if (value & 0xff800000) == 0x7e800000 {
            return vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(state);
        } else if (value & 0xff800000) == 0x5e000000 {
            return vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_bulk_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_bulk_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_bulk_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e000000 {
            return vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_conv_float_bulk_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x5e800000 {
            return vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00008800 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_mul_product_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x0000c400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f800000) == 0x2e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_fp_2008_decode(state);
                }
            } else if (value & 0x3f800000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_fp_2008_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_fp_2008_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_fp_2008_decode(state);
                }
            }
        }
    } else if (value & 0xfffffc00) == 0x5e280800 {
        return vector_crypto_sha2op_sha1hash_decode(state);
    } else if (value & 0x3ffffc00) == 0x0e799800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_round_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000d400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f800000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_fp_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_fp_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_sub_fp_simd_decode(state);
                }
            } else if (value & 0xff800000) == 0x7e800000 {
                return vector_arithmetic_binary_uniform_sub_fp_sisd_decode(state);
            } else if (value & 0x3f800000) == 0x2e800000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_sub_fp_simd_decode(state);
                }
            }
        }
    } else if (value & 0xfffffc00) == 0x5e281800 {
        return vector_crypto_sha2op_sha1sched1_decode(state);
    } else if (value & 0x003ffc00) == 0x00200800 {
        if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_rev_decode(state);
            }
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_rev_decode(state);
            }
        }
    } else if (value & 0xfffffc00) == 0x4e285800 {
        return vector_crypto_aes_round_decode(state);
    } else if (value & 0x0000fc00) == 0x00006400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_single_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_single_decode(state);
                }
            }
        }
        if (value & 0xff800000) == 0x7f000000 {
            return vector_shift_leftsat_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_leftsat_simd_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x0020c800 {
        if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_float_bulk_simd_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e800000 {
            return vector_arithmetic_unary_cmp_float_bulk_sisd_decode(state);
        } else if (value & 0xff800000) == 0x5e800000 {
            return vector_arithmetic_unary_cmp_float_bulk_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_float_bulk_simd_decode(state);
            }
        }
    } else if (value & 0x00001c00) == 0x00000800 {
        if (value & 0xffffc000) == 0xdac10000 {
            return integer_pac_pacda_dp_1src_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00004800 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_maxmin_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x0ef8f800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_diffneg_fp16_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0e30f800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_reduce_fp16max_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00203800 {
        if (value & 0xff000000) == 0x5e000000 {
            return vector_arithmetic_unary_add_saturating_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_add_saturating_simd_decode(state);
            }
        } else if (value & 0xff000000) == 0x7e000000 {
            return vector_arithmetic_unary_add_saturating_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_add_saturating_simd_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x00007000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_diff_decode(state);
                }
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_ld_decode(state);
                }
            } else if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_diff_decode(state);
                }
            }
        }
    } else if (value & 0x3ffffc00) == 0x0e79d800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_int_simd_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0e798800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_round_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x7e79c800 {
        return vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode(state);
    } else if (value & 0xfffffc00) == 0x5ef9b800 {
        return vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(state);
    } else if (value & 0x003ffc00) == 0x00390000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00200000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000d000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_double_simd_decode(state);
                }
            } else if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_disparate_mul_double_sisd_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x0eb0f800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_reduce_fp16max_simd_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x7e79a800 {
        return vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(state);
    } else if (value & 0x3ffffc00) == 0x0e79a800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x0020f800 {
        if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_diffneg_float_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_diffneg_float_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x2e799800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_round_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000bc00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_add_wrapping_pair_decode(state);
                }
            }
        }
    } else if (value & 0x003ffc00) == 0x0031b800 {
        if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_reduce_add_simd_decode(state);
            }
        } else if (value & 0xff000000) == 0x5e000000 {
            return vector_reduce_add_sisd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x0020b800 {
        if (value & 0xff000000) == 0x5e000000 {
            return vector_arithmetic_unary_diffneg_int_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_diffneg_int_simd_decode(state);
            }
        } else if (value & 0xff000000) == 0x7e000000 {
            return vector_arithmetic_unary_diffneg_int_sisd_decode(state);
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_diffneg_int_simd_decode(state);
            }
        }
    } else if (value & 0xfffffc00) == 0x7ef9d800 {
        return vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode(state);
    } else if (value & 0x0000fc00) == 0x0000dc00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f800000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_fp_product_decode(state);
                }
            } else if (value & 0x3f800000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode(state);
                }
            } else if (value & 0xff800000) == 0x5e000000 {
                return vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x00380000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x00001c00) == 0x00000000 {
        if (value & 0xffffc000) == 0xdac10000 {
            return integer_pac_pacia_dp_1src_decode(state);
        }
        if (value & 0x00008000) == 0x00000000 {
            if (value & 0x3fe00000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_table_decode(state);
                }
            }
        }
    } else if (value & 0xfffffc00) == 0x5e79d800 {
        return vector_arithmetic_unary_fp16_conv_int_sisd_decode(state);
    } else if (value & 0x003ffc00) == 0x0025c000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_round_frint_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x7e79d800 {
        return vector_arithmetic_unary_fp16_conv_int_sisd_decode(state);
    } else if (value & 0x0000fc00) == 0x00003800 {
        if (value & 0x00200000) == 0x00000000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_permute_zip_decode(state);
                }
            }
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_addsub_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x0000ac00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_pair_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_pair_decode(state);
                }
            }
        }
    } else if (value & 0x003ffc00) == 0x0021c000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_unary_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x2ef99800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_round_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00216800 {
        if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_xtn_simd_decode(state);
            }
        } else if (value & 0x3f800000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_narrow_decode(state);
            }
        } else if (value & 0xff800000) == 0x7e000000 {
            return vector_arithmetic_unary_float_xtn_sisd_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x7ef8c800 {
        return vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode(state);
    } else if (value & 0x0000fc00) == 0x00000800 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_mul_product_decode(state);
            }
        } else if (value & 0x7fe00000) == 0x1ac00000 {
            return integer_arithmetic_div_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00009000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x5e000000 {
                return vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_dmacc_simd_decode(state);
                }
            }
        }
    } else if (value & 0x003ffc00) == 0x00218800 {
        if (value & 0x3f800000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_round_decode(state);
            }
        } else if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_round_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_float_round_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x2e79a800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x5e79c800 {
        return vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode(state);
    } else if (value & 0x003ffc00) == 0x00204000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_unary_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0eb0c800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_reduce_fp16maxnm_simd_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000a400 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_pair_decode(state);
                }
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_pair_decode(state);
                }
            }
        }
        if (value & 0x3f800000) == 0x0f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_leftlong_decode(state);
            }
        } else if (value & 0x3f800000) == 0x2f000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_shift_leftlong_decode(state);
            }
        }
    } else if (value & 0x0000fc00) == 0x00006c00 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_single_decode(state);
                }
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_uniform_maxmin_single_decode(state);
                }
            }
        }
    } else if (value & 0xfffffc00) == 0x5ef8e800 {
        return vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode(state);
    } else if (value & 0x003ffc00) == 0x0030d800 {
        if (value & 0xff800000) == 0x7e000000 {
            return vector_reduce_fpadd_sisd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00206800 {
        if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_add_pairwise_decode(state);
            }
        } else if (value & 0x3f000000) == 0x2e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_add_pairwise_decode(state);
            }
        }
    } else if (value & 0x00007c00) == 0x00004000 {
        if (value & 0x003e0000) == 0x00280000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_round_frint_32_64_decode(state);
            }
        } else if (value & 0x003e0000) == 0x00220000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_convert_fp_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x2ef9b800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00214000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_unary_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0ef8c800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_cmp_fp16_bulk_simd_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x2e605800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_rbit_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x5ef9f800 {
        return vector_arithmetic_unary_special_frecpxfp16_decode(state);
    } else if (value & 0x0000fc00) == 0x00006800 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1e000000 {
                return float_arithmetic_maxmin_decode(state);
            }
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_permute_transpose_decode(state);
                }
            }
        }
    } else if (value & 0x003ffc00) == 0x00280000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x00005000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x38000000 {
                if (value & 0x80000000) == 0x80000000 {
                    return memory_atomicops_ld_decode(state);
                }
            } else if (value & 0xff000000) == 0x78000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x2e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_diff_decode(state);
                }
            } else if (value & 0xff000000) == 0x38000000 {
                return memory_atomicops_ld_decode(state);
            } else if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_diff_decode(state);
                }
            }
        } else if (value & 0xffe00000) == 0x5e000000 {
            return vector_crypto_sha3op_sha256hash_decode(state);
        }
    } else if (value & 0x00000400) == 0x00000400 {
        if (value & 0x0000e000) == 0x0000c000 {
            if (value & 0x00200000) == 0x00000000 {
                if (value & 0x3f000000) == 0x2e000000 {
                    if (value & 0x80000000) == 0x00000000 {
                        return vector_arithmetic_binary_uniform_mul_fp_complex_decode(state);
                    }
                }
            }
        }
        if (value & 0x00008000) == 0x00000000 {
            if (value & 0xffe00000) == 0x6e000000 {
                return vector_transfer_vector_insert_decode(state);
            }
        }
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0xf8000000 {
                return memory_single_general_immediate_signed_pac_decode(state);
            }
        }
    } else if (value & 0x3ffffc00) == 0x2ef9d800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_special_sqrtest_fp16_simd_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0ef9d800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_special_recip_fp16_simd_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x5e30c800 {
        return vector_reduce_fp16maxnm_sisd_decode(state);
    } else if (value & 0x003ffc00) == 0x00240000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0ef99800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_round_decode(state);
        }
    } else if (value & 0x00001c00) == 0x00001000 {
        if (value & 0xffffc000) == 0xdac10000 {
            return integer_pac_autia_dp_1src_decode(state);
        }
        if (value & 0x00008000) == 0x00000000 {
            if (value & 0x3fe00000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_transfer_vector_table_decode(state);
                }
            }
        }
    } else if (value & 0x003ffc00) == 0x00205800 {
        if (value & 0x3f000000) == 0x0e000000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cnt_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x0020e800 {
        if (value & 0xff800000) == 0x5e800000 {
            return vector_arithmetic_unary_cmp_float_lessthan_sisd_decode(state);
        } else if (value & 0x3f800000) == 0x0e800000 {
            if (value & 0x80000000) == 0x00000000 {
                return vector_arithmetic_unary_cmp_float_lessthan_simd_decode(state);
            }
        }
    } else if (value & 0x003ffc00) == 0x00254000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_round_frint_decode(state);
        }
    } else if (value & 0x0000fc00) == 0x0000e000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0x3f000000) == 0x0e000000 {
                if (value & 0x80000000) == 0x00000000 {
                    return vector_arithmetic_binary_disparate_mul_poly_decode(state);
                }
            }
        }
    } else if (value & 0xfffffc00) == 0x7ef9a800 {
        return vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(state);
    } else if (value & 0xfffffc00) == 0x7e79b800 {
        return vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(state);
    } else if (value & 0xfffffc00) == 0x5ef9a800 {
        return vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(state);
    } else if (value & 0xfffffc00) == 0xd5022400 {
        return integer_tags_mcgettagarray_decode(state);
    } else if (value & 0x7ffffc00) == 0x5ac01000 {
        return integer_arithmetic_cnt_decode(state);
    } else if (value & 0x003ffc00) == 0x00290000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x1e7e0000 {
        return float_convert_int_decode(state);
    } else if (value & 0x7ffffc00) == 0x5ac00400 {
        return integer_arithmetic_rev_decode(state);
    } else if (value & 0x003ffc00) == 0x00274000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_round_frint_decode(state);
        }
    } else if (value & 0x00001c00) == 0x00001800 {
        if (value & 0xffffc000) == 0xdac10000 {
            return integer_pac_autda_dp_1src_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x0e30c800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_reduce_fp16maxnm_simd_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x5e79b800 {
        return vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(state);
    } else if (value & 0x3ffffc00) == 0x2e798800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_round_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x2e79b800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00220000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0x00001c00) == 0x00000400 {
        if (value & 0xffffc000) == 0xdac10000 {
            return integer_pac_pacib_dp_1src_decode(state);
        }
    } else if (value & 0xfffffc00) == 0xd5022000 {
        return integer_tags_mcsettagarray_decode(state);
    } else if (value & 0x3ffffc00) == 0x0ef9b800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(state);
        }
    } else if (value & 0x7ffffc00) == 0x5ac01400 {
        return integer_arithmetic_cnt_decode(state);
    } else if (value & 0xfffffc00) == 0x4e286800 {
        return vector_crypto_aes_mix_decode(state);
    } else if (value & 0x3ffffc00) == 0x2ef9f800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_special_sqrtfp16_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00310000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x5ef8c800 {
        return vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode(state);
    } else if (value & 0x003ffc00) == 0x00210000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_int_decode(state);
        }
    } else if (value & 0xfffffc00) == 0x5e30f800 {
        return vector_reduce_fp16max_sisd_decode(state);
    } else if (value & 0x3ffffc00) == 0x0e79c800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode(state);
        }
    } else if (value & 0x003ffc00) == 0x00244000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_round_frint_decode(state);
        }
    } else if (value & 0x3ffffc00) == 0x2e205800 {
        if (value & 0x80000000) == 0x00000000 {
            return vector_arithmetic_unary_not_decode(state);
        }
    } else if (value & 0x7ffffc00) == 0x5ac00000 {
        return integer_arithmetic_rbit_decode(state);
    } else if (value & 0x003ffc00) == 0x0027c000 {
        if (value & 0xff000000) == 0x1e000000 {
            return float_arithmetic_round_frint_decode(state);
        }
    }
    if (value & 0x00fff800) == 0x003f0800 {
        if (value & 0xfe000000) == 0xd6000000 {
            return branch_unconditional_register_decode(state);
        }
    } else if (value & 0x7ffff800) == 0x5ac00800 {
        return integer_arithmetic_rev_decode(state);
    } else if (value & 0x00fff800) == 0x001f0800 {
        if (value & 0xfe000000) == 0xd6000000 {
            return branch_unconditional_register_decode(state);
        }
    }
    if (value & 0x0000f000) == 0x00004000 {
        if (value & 0x3fe00000) == 0x0c800000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(state);
            }
        } else if (value & 0x7fe00000) == 0x1ac00000 {
            return integer_crc_decode(state);
        } else if (value & 0x3fe00000) == 0x0cc00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(state);
            }
        }
    } else if (value & 0x3ffff000) == 0x0c408000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(state);
        }
    } else if (value & 0x3ffff000) == 0x0c008000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(state);
        }
    } else if (value & 0x0000f000) == 0x0000e000 {
        if (value & 0x3fe00000) == 0x0de00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0dc00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        }
    } else if (value & 0x3ffff000) == 0x0c400000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(state);
        }
    } else if (value & 0x3ffff000) == 0x0d60c000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
        }
    } else if (value & 0x0000f000) == 0x00008000 {
        if (value & 0x3fe00000) == 0x0cc00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0c800000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(state);
            }
        }
    } else if (value & 0x0000f000) == 0x00005000 {
        if (value & 0x7fe00000) == 0x1ac00000 {
            return integer_crc_decode(state);
        }
    } else if (value & 0x3ffff000) == 0x0d40e000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
        }
    } else if (value & 0x3ffff000) == 0x0c404000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(state);
        }
    } else if (value & 0x0000f000) == 0x00000000 {
        if (value & 0x3fe00000) == 0x0cc00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0c800000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(state);
            }
        }
    } else if (value & 0x3ffff000) == 0x0c000000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(state);
        }
    } else if (value & 0x0000f000) == 0x0000c000 {
        if (value & 0x3fe00000) == 0x0dc00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0de00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        }
    } else if (value & 0x3ffff000) == 0x0d40c000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
        }
    } else if (value & 0x3ffff000) == 0x0c004000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(state);
        }
    } else if (value & 0x3ffff000) == 0x0d60e000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
        }
    }
    if (value & 0x00002000) == 0x00002000 {
        if (value & 0x3fff0000) == 0x0d600000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fff0000) == 0x0d200000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fff0000) == 0x0c400000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(state);
            }
        } else if (value & 0x3fff0000) == 0x0d000000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fff0000) == 0x0d400000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fff0000) == 0x0c000000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(state);
            }
        }
        if (value & 0x3fe00000) == 0x0da00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0de00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0d800000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0c800000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0dc00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0cc00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(state);
            }
        }
    } else if (value & 0x00002000) == 0x00000000 {
        if (value & 0x3fff0000) == 0x0d200000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fff0000) == 0x0d000000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fff0000) == 0x0d600000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fff0000) == 0x0d400000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_nowb_memory_vector_single_nowb__decode(state);
            }
        }
        if (value & 0x3fe00000) == 0x0da00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0d800000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0dc00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        } else if (value & 0x3fe00000) == 0x0de00000 {
            if (value & 0x80000000) == 0x00000000 {
                return memory_vector_single_postinc_memory_vector_single_nowb__decode(state);
            }
        }
    }
    if (value & 0x00008000) == 0x00000000 {
        if (value & 0xffe00000) == 0x48400000 {
            return memory_exclusive_single_decode(state);
        } else if (value & 0xffe00000) == 0x9bc00000 {
            return integer_arithmetic_mul_widening_64128hi_decode(state);
        } else if (value & 0x3fe00000) == 0x08600000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_exclusive_pair_decode(state);
            }
        } else if (value & 0xffe00000) == 0x9ba00000 {
            return integer_arithmetic_mul_widening_3264_decode(state);
        } else if (value & 0xffe00000) == 0x9b200000 {
            return integer_arithmetic_mul_widening_3264_decode(state);
        } else if (value & 0xffe00000) == 0x08c00000 {
            return memory_ordered_decode(state);
        } else if (value & 0xffe00000) == 0x08400000 {
            return memory_exclusive_single_decode(state);
        } else if (value & 0x3fe00000) == 0x08400000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_exclusive_single_decode(state);
            }
        } else if (value & 0xffe00000) == 0x08800000 {
            return memory_ordered_decode(state);
        } else if (value & 0x3fe00000) == 0x08c00000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_ordered_decode(state);
            }
        } else if (value & 0x7fe00000) == 0x1b000000 {
            return integer_arithmetic_mul_uniform_addsub_decode(state);
        } else if (value & 0x3fe00000) == 0x08000000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_exclusive_single_decode(state);
            }
        } else if (value & 0xffe00000) == 0x48800000 {
            return memory_ordered_decode(state);
        } else if (value & 0xffe00000) == 0x08000000 {
            return memory_exclusive_single_decode(state);
        } else if (value & 0x3fe00000) == 0x08200000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_exclusive_pair_decode(state);
            }
        } else if (value & 0xffe00000) == 0x48c00000 {
            return memory_ordered_decode(state);
        } else if (value & 0xffe00000) == 0x9b400000 {
            return integer_arithmetic_mul_widening_64128hi_decode(state);
        } else if (value & 0x3fe00000) == 0x08800000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_ordered_decode(state);
            }
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0xff000000) == 0x1f000000 {
                return float_arithmetic_mul_addsub_decode(state);
            }
        } else if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1f000000 {
                return float_arithmetic_mul_addsub_decode(state);
            }
        } else if (value & 0xffe00000) == 0x48000000 {
            return memory_exclusive_single_decode(state);
        }
    } else if (value & 0x00008000) == 0x00008000 {
        if (value & 0x00200000) == 0x00200000 {
            if (value & 0xff000000) == 0x1f000000 {
                return float_arithmetic_mul_addsub_decode(state);
            }
        } else if (value & 0xffe00000) == 0x48800000 {
            return memory_ordered_decode(state);
        } else if (value & 0x3fe00000) == 0x08c00000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_ordered_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x08400000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_exclusive_single_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x08800000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_ordered_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x08000000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_exclusive_single_decode(state);
            }
        } else if (value & 0xffe00000) == 0x9b200000 {
            return integer_arithmetic_mul_widening_3264_decode(state);
        } else if (value & 0xffe00000) == 0x48000000 {
            return memory_exclusive_single_decode(state);
        } else if (value & 0x7fe00000) == 0x1b000000 {
            return integer_arithmetic_mul_uniform_addsub_decode(state);
        } else if (value & 0xffe00000) == 0x08800000 {
            return memory_ordered_decode(state);
        } else if (value & 0xffe00000) == 0x08000000 {
            return memory_exclusive_single_decode(state);
        } else if (value & 0xffe00000) == 0x08400000 {
            return memory_exclusive_single_decode(state);
        } else if (value & 0xffe00000) == 0x48400000 {
            return memory_exclusive_single_decode(state);
        } else if (value & 0xffe00000) == 0x48c00000 {
            return memory_ordered_decode(state);
        } else if (value & 0x3fe00000) == 0x08200000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_exclusive_pair_decode(state);
            }
        } else if (value & 0x3fe00000) == 0x08600000 {
            if (value & 0x80000000) == 0x80000000 {
                return memory_exclusive_pair_decode(state);
            }
        } else if (value & 0xffe00000) == 0x08c00000 {
            return memory_ordered_decode(state);
        } else if (value & 0x00200000) == 0x00000000 {
            if (value & 0xff000000) == 0x1f000000 {
                return float_arithmetic_mul_addsub_decode(state);
            }
        } else if (value & 0xffe00000) == 0x9ba00000 {
            return integer_arithmetic_mul_widening_3264_decode(state);
        }
    }
    if (value & 0x003f0000) == 0x00020000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_fix_decode(state);
        }
    } else if (value & 0x003f0000) == 0x00030000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_fix_decode(state);
        }
    } else if (value & 0x003f0000) == 0x00180000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_fix_decode(state);
        }
    } else if (value & 0x003f0000) == 0x00190000 {
        if (value & 0x7f000000) == 0x1e000000 {
            return float_convert_fix_decode(state);
        }
    }
    if (value & 0xfff80000) == 0xd5280000 {
        return system_sysops_decode(state);
    } else if (value & 0xfff80000) == 0xd5080000 {
        return system_sysops_decode(state);
    }
    if (value & 0xfff00000) == 0xd5300000 {
        return system_register_system_decode(state);
    } else if (value & 0xfff00000) == 0xd5100000 {
        return system_register_system_decode(state);
    }
    if (value & 0xffe00000) == 0xce800000 {
        return vector_crypto_sha3_xar_decode(state);
    } else if (value & 0x7fe00000) == 0x2b200000 {
        return integer_arithmetic_addsub_extendedreg_decode(state);
    } else if (value & 0x00200000) == 0x00200000 {
        if (value & 0x7f000000) == 0x0a000000 {
            return integer_logical_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x6a000000 {
            return integer_logical_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x2a000000 {
            return integer_logical_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x4a000000 {
            return integer_logical_shiftedreg_decode(state);
        }
    } else if (value & 0x00200000) == 0x00000000 {
        if (value & 0x7f800000) == 0x13800000 {
            return integer_insext_extract_immediate_decode(state);
        }
        if (value & 0x7f000000) == 0x6a000000 {
            return integer_logical_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x2a000000 {
            return integer_logical_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x0b000000 {
            return integer_arithmetic_addsub_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x4a000000 {
            return integer_logical_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x6b000000 {
            return integer_arithmetic_addsub_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x4b000000 {
            return integer_arithmetic_addsub_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x0a000000 {
            return integer_logical_shiftedreg_decode(state);
        } else if (value & 0x7f000000) == 0x2b000000 {
            return integer_arithmetic_addsub_shiftedreg_decode(state);
        }
    } else if (value & 0x7fe00000) == 0x6b200000 {
        return integer_arithmetic_addsub_extendedreg_decode(state);
    } else if (value & 0x7fe00000) == 0x0b200000 {
        return integer_arithmetic_addsub_extendedreg_decode(state);
    } else if (value & 0x7fe00000) == 0x4b200000 {
        return integer_arithmetic_addsub_extendedreg_decode(state);
    }
    if (value & 0xffc00000) == 0x69800000 {
        return integer_tags_mcsettaganddatapairpre_decode(state);
    } else if (value & 0xffc00000) == 0x91800000 {
        return integer_tags_mcaddtag_decode(state);
    } else if (value & 0x7fc00000) == 0x29000000 {
        return memory_pair_general_offset_memory_pair_general_postidx__decode(state);
    } else if (value & 0x7fc00000) == 0x28400000 {
        return memory_pair_general_noalloc_memory_pair_general_noalloc__decode(state);
    } else if (value & 0x3fc00000) == 0x39400000 {
        if (value & 0x80000000) == 0x80000000 {
            return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        }
    } else if (value & 0x7fc00000) == 0x29400000 {
        return memory_pair_general_offset_memory_pair_general_postidx__decode(state);
    } else if (value & 0xffc00000) == 0x69000000 {
        return integer_tags_mcsettaganddatapair_decode(state);
    } else if (value & 0x3fc00000) == 0x2d000000 {
        return memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode(state);
    } else if (value & 0x3fc00000) == 0x2c400000 {
        return memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode(state);
    } else if (value & 0x7fc00000) == 0x29800000 {
        return memory_pair_general_preidx_memory_pair_general_postidx__decode(state);
    } else if (value & 0x3fc00000) == 0x2c800000 {
        return memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode(state);
    } else if (value & 0xffc00000) == 0x69400000 {
        return memory_pair_general_offset_memory_pair_general_postidx__decode(state);
    } else if (value & 0x3fc00000) == 0x2c000000 {
        return memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode(state);
    } else if (value & 0xffc00000) == 0x69c00000 {
        return memory_pair_general_preidx_memory_pair_general_postidx__decode(state);
    } else if (value & 0xffc00000) == 0x68800000 {
        return integer_tags_mcsettaganddatapairpost_decode(state);
    } else if (value & 0xffc00000) == 0xf9800000 {
        return memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode(
            state,
        );
    } else if (value & 0x3fc00000) == 0x2d400000 {
        return memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode(state);
    } else if (value & 0x3fc00000) == 0x39000000 {
        if (value & 0x80000000) == 0x80000000 {
            return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
                state,
            );
        }
    } else if (value & 0x3fc00000) == 0x2cc00000 {
        return memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode(state);
    } else if (value & 0xffc00000) == 0x79000000 {
        return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
            state,
        );
    } else if (value & 0x7fc00000) == 0x28800000 {
        return memory_pair_general_postidx_memory_pair_general_postidx__decode(state);
    } else if (value & 0xffc00000) == 0xd1800000 {
        return integer_tags_mcsubtag_decode(state);
    } else if (value & 0x7fc00000) == 0x28000000 {
        return memory_pair_general_noalloc_memory_pair_general_noalloc__decode(state);
    } else if (value & 0xffc00000) == 0x68c00000 {
        return memory_pair_general_postidx_memory_pair_general_postidx__decode(state);
    } else if (value & 0x3fc00000) == 0x2dc00000 {
        return memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode(state);
    } else if (value & 0x7fc00000) == 0x29c00000 {
        return memory_pair_general_preidx_memory_pair_general_postidx__decode(state);
    } else if (value & 0x7fc00000) == 0x28c00000 {
        return memory_pair_general_postidx_memory_pair_general_postidx__decode(state);
    } else if (value & 0x00400000) == 0x00400000 {
        if (value & 0x3f000000) == 0x3d000000 {
            return memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode(
                state,
            );
        }
    } else if (value & 0xffc00000) == 0x39000000 {
        return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
            state,
        );
    } else if (value & 0x00400000) == 0x00000000 {
        if (value & 0x3f000000) == 0x3d000000 {
            return memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode(
                state,
            );
        }
    } else if (value & 0xffc00000) == 0xb9800000 {
        return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
            state,
        );
    } else if (value & 0xffc00000) == 0x39400000 {
        return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
            state,
        );
    } else if (value & 0x3fc00000) == 0x2d800000 {
        return memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode(state);
    } else if (value & 0xffc00000) == 0x79400000 {
        return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
            state,
        );
    }
    if (value & 0xff800000) == 0x79800000 {
        return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
            state,
        );
    } else if (value & 0x7f800000) == 0x72000000 {
        return integer_logical_immediate_decode(state);
    } else if (value & 0xff800000) == 0x39800000 {
        return memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
            state,
        );
    } else if (value & 0x7f800000) == 0x12800000 {
        return integer_insext_insert_movewide_decode(state);
    } else if (value & 0x7f800000) == 0x13000000 {
        return integer_bitfield_decode(state);
    } else if (value & 0x7f800000) == 0x52000000 {
        return integer_logical_immediate_decode(state);
    } else if (value & 0x7f800000) == 0x52800000 {
        return integer_insext_insert_movewide_decode(state);
    } else if (value & 0x7f800000) == 0x72800000 {
        return integer_insext_insert_movewide_decode(state);
    } else if (value & 0x7f800000) == 0x33000000 {
        return integer_bitfield_decode(state);
    } else if (value & 0x7f800000) == 0x12000000 {
        return integer_logical_immediate_decode(state);
    } else if (value & 0x7f800000) == 0x53000000 {
        return integer_bitfield_decode(state);
    } else if (value & 0x7f800000) == 0x32000000 {
        return integer_logical_immediate_decode(state);
    }
    if (value & 0x7f000000) == 0x37000000 {
        return branch_conditional_test_decode(state);
    } else if (value & 0x7f000000) == 0x51000000 {
        return integer_arithmetic_addsub_immediate_decode(state);
    } else if (value & 0x7f000000) == 0x31000000 {
        return integer_arithmetic_addsub_immediate_decode(state);
    } else if (value & 0x1f000000) == 0x10000000 {
        if (value & 0x80000000) == 0x00000000 {
            return integer_arithmetic_address_pcrel_decode(state);
        } else if (value & 0x80000000) == 0x80000000 {
            return integer_arithmetic_address_pcrel_decode(state);
        }
    } else if (value & 0x7f000000) == 0x35000000 {
        return branch_conditional_compare_decode(state);
    } else if (value & 0x7f000000) == 0x71000000 {
        return integer_arithmetic_addsub_immediate_decode(state);
    } else if (value & 0x7f000000) == 0x11000000 {
        return integer_arithmetic_addsub_immediate_decode(state);
    } else if (value & 0x3f000000) == 0x1c000000 {
        return memory_literal_simdfp_decode(state);
    } else if (value & 0xff000000) == 0x98000000 {
        return memory_literal_general_decode(state);
    } else if (value & 0x7f000000) == 0x34000000 {
        return branch_conditional_compare_decode(state);
    } else if (value & 0xff000000) == 0xd8000000 {
        return memory_literal_general_decode(state);
    } else if (value & 0x3f000000) == 0x18000000 {
        if (value & 0x80000000) == 0x00000000 {
            return memory_literal_general_decode(state);
        }
    } else if (value & 0x7f000000) == 0x36000000 {
        return branch_conditional_test_decode(state);
    }
    if (value & 0xfc000000) == 0x14000000 {
        return branch_unconditional_immediate_decode(state);
    } else if (value & 0xfc000000) == 0x94000000 {
        return branch_unconditional_immediate_decode(state);
    }
    ExecuteResult::UndefinedInstruction
}
fn integer_arithmetic_mul_widening_3264_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_mul_widening_3264_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_addsub_shiftedreg_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_addsub_shiftedreg_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_maxmin_pair_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_maxmin_pair_decode\"");
    ExecuteResult::Ok
}
fn memory_atomicops_st_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_atomicops_st_decode\"");
    ExecuteResult::Ok
}
fn memory_pair_general_preidx_memory_pair_general_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_pair_general_preidx_memory_pair_general_postidx__decode\"");
    ExecuteResult::Ok
}
fn memory_atomicops_ld_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_atomicops_ld_decode\"");
    ExecuteResult::Ok
}
fn vector_logical_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_logical_decode\"");
    ExecuteResult::Ok
}
fn vector_fp16_movi_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_fp16_movi_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettaganddatapair_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettaganddatapair_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_autib_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_autib_dp_1src_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_cmp_int_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_cmp_int_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_sub_fp16_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_sub_fp16_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_fp16_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_fp16_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_logical_andorr_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_logical_andorr_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_halving_truncating_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_halving_truncating_decode\"");
    ExecuteResult::Ok
}
fn memory_ordered_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_ordered_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_int_bulk_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_int_bulk_sisd_decode\"");
    ExecuteResult::Ok
}
fn system_hints_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_hints_decode\"");
    ExecuteResult::Ok
}
fn system_barriers_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_barriers_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_saturating_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_saturating_sisd_decode\"");
    ExecuteResult::Ok
}
fn memory_vector_single_postinc_memory_vector_single_nowb__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_vector_single_postinc_memory_vector_single_nowb__decode\"");
    ExecuteResult::Ok
}
fn float_convert_fix_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_convert_fix_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_fp16_round_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_fp16_round_decode\"");
    ExecuteResult::Ok
}
fn float_arithmetic_addsub_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_arithmetic_addsub_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_addsub_carry_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_addsub_carry_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_wrapping_pair_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_wrapping_pair_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcinserttagmask_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcinserttagmask_decode\"");
    ExecuteResult::Ok
}
fn memory_pair_general_offset_memory_pair_general_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_pair_general_offset_memory_pair_general_postidx__decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fp16maxnm_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fp16maxnm_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_integer_dup_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_integer_dup_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_addsub_immediate_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_addsub_immediate_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_diff_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_diff_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_diffneg_float_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_diffneg_float_decode\"");
    ExecuteResult::Ok
}
fn memory_vector_single_nowb_memory_vector_single_nowb__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_vector_single_nowb_memory_vector_single_nowb__decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_conv_float_bulk_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_conv_float_bulk_sisd_decode\"");
    ExecuteResult::Ok
}
fn memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_fp_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_fp_sisd_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode\""
    );
    ExecuteResult::Ok
}
fn memory_exclusive_single_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_exclusive_single_decode\"");
    ExecuteResult::Ok
}
fn float_arithmetic_mul_addsub_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_arithmetic_mul_addsub_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_diffneg_int_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_diffneg_int_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcaddtag_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcaddtag_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_addsub_extendedreg_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_addsub_extendedreg_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_cmp_int_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_cmp_int_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_conv_int_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_conv_int_sisd_decode\"");
    ExecuteResult::Ok
}
fn memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode\"");
    ExecuteResult::Ok
}
fn memory_atomicops_swp_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_atomicops_swp_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_right_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_right_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_conditional_compare_immediate_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_conditional_compare_immediate_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode\""
    );
    ExecuteResult::Ok
}
fn vector_reduce_fp16max_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fp16max_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_maxmin_fp_2008_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_maxmin_fp_2008_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_mul_uniform_addsub_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_mul_uniform_addsub_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fp16add_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fp16add_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_cmp_fp_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_cmp_fp_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_conv_float_bulk_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_conv_float_bulk_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode\"");
    ExecuteResult::Ok
}
fn float_arithmetic_unary_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_arithmetic_unary_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_high_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_high_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_add_pairwise_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_add_pairwise_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp_complex_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp_complex_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fp16maxnm_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fp16maxnm_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagpairpre_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagpairpre_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_int_bulk_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_int_bulk_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagpairandzerodatapost_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagpairandzerodatapost_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_extract_sqxtun_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_extract_sqxtun_sisd_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode\""
    );
    ExecuteResult::Ok
}
fn vector_reduce_intmax_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_intmax_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha3op_sha1hash_majority_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha3op_sha1hash_majority_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha3op_sha256hash_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha3op_sha256hash_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_sub_fp_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_sub_fp_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fpmax_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fpmax_simd_decode\"");
    ExecuteResult::Ok
}
fn memory_atomicops_cas_single_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_atomicops_cas_single_decode\"");
    ExecuteResult::Ok
}
fn system_monitors_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_monitors_decode\"");
    ExecuteResult::Ok
}
fn float_compare_cond_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_compare_cond_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_recip_fp16_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_recip_fp16_sisd_decode\"");
    ExecuteResult::Ok
}
fn system_register_system_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_register_system_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_fp_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_fp_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_conditional_compare_register_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_conditional_compare_register_decode\"");
    ExecuteResult::Ok
}
fn integer_conditional_select_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_conditional_select_decode\"");
    ExecuteResult::Ok
}
fn memory_literal_general_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_literal_general_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_cmp_fp_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_cmp_fp_simd_decode\"");
    ExecuteResult::Ok
}
fn memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_extract_sat_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_extract_sat_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_aes_mix_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_aes_mix_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_autdb_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_autdb_dp_1src_decode\"");
    ExecuteResult::Ok
}
fn memory_exclusive_pair_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_exclusive_pair_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_integer_move_unsigned_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_integer_move_unsigned_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_shift_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_shift_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_round_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_round_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_rightnarrow_uniform_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_rightnarrow_uniform_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp_fused_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp_fused_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha3op_sha256sched1_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha3op_sha256sched1_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode\"");
    ExecuteResult::Ok
}
fn branch_unconditional_register_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"branch_unconditional_register_decode\"");
    ExecuteResult::Ok
}
fn system_register_cpsr_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_register_cpsr_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_xtn_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_xtn_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_diffneg_sat_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_diffneg_sat_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_float_bulk_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_float_bulk_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_addsub_narrow_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_addsub_narrow_decode\"");
    ExecuteResult::Ok
}
fn integer_insext_insert_movewide_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_insext_insert_movewide_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_fp16_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_fp16_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_fp_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_fp_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_shift_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_shift_simd_decode\"");
    ExecuteResult::Ok
}
fn branch_conditional_compare_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"branch_conditional_compare_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_register_memory_single_general_register__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_general_register_memory_single_general_register__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode\"");
    ExecuteResult::Ok
}
fn memory_single_simdfp_register_memory_single_simdfp_register__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_single_simdfp_register_memory_single_simdfp_register__decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_add_saturating_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_add_saturating_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_logical_bsleor_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_logical_bsleor_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_diffneg_fp16_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_diffneg_fp16_decode\"");
    ExecuteResult::Ok
}
fn memory_pair_general_postidx_memory_pair_general_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_pair_general_postidx_memory_pair_general_postidx__decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_aes_round_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_aes_round_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_recpsfp16_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_recpsfp16_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_frecpxfp16_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_frecpxfp16_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_rev_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_rev_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp16_product_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp16_product_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp16_fused_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp16_fused_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_int_lessthan_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_int_lessthan_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha2op_sha1hash_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha2op_sha1hash_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_sqrtest_int_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_sqrtest_int_decode\"");
    ExecuteResult::Ok
}
fn memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode\"");
    ExecuteResult::Ok
}
fn integer_logical_immediate_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_logical_immediate_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_vector_permute_unzip_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_vector_permute_unzip_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_fp16_bulk_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_fp16_bulk_simd_decode\"");
    ExecuteResult::Ok
}
fn memory_pair_general_noalloc_memory_pair_general_noalloc__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_pair_general_noalloc_memory_pair_general_noalloc__decode\"");
    ExecuteResult::Ok
}
fn vector_shift_leftsat_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_leftsat_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_vector_cpydup_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_vector_cpydup_simd_decode\"");
    ExecuteResult::Ok
}
fn branch_unconditional_immediate_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"branch_unconditional_immediate_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_leftlong_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_leftlong_decode\"");
    ExecuteResult::Ok
}
fn memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_add_saturating_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_add_saturating_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_cnt_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_cnt_decode\"");
    ExecuteResult::Ok
}
fn branch_conditional_test_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"branch_conditional_test_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_vector_permute_transpose_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_vector_permute_transpose_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_addsub_long_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_addsub_long_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_float_bulk_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_float_bulk_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_rightnarrow_nonuniform_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_rightnarrow_nonuniform_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_logical_shiftedreg_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_logical_shiftedreg_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_vector_permute_zip_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_vector_permute_zip_decode\"");
    ExecuteResult::Ok
}
fn integer_crc_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_crc_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_conv_float_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_conv_float_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_leftsat_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_leftsat_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_left_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_left_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_recip_float_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_recip_float_sisd_decode\"");
    ExecuteResult::Ok
}
fn float_arithmetic_maxmin_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_arithmetic_maxmin_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_sub_int_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_sub_int_decode\"");
    ExecuteResult::Ok
}
fn float_convert_int_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_convert_int_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_mul_accum_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_mul_accum_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_extract_nosat_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_extract_nosat_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_fp16_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_fp16_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_right_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_right_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_rightinsert_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_rightinsert_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_autda_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_autda_dp_1src_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_fp_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_fp_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_diff_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_diff_decode\"");
    ExecuteResult::Ok
}
fn memory_orderedrcpc_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_orderedrcpc_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettaganddatapairpre_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettaganddatapairpre_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_extract_sat_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_extract_sat_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_divfp16_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_divfp16_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagandzerodatapre_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagandzerodatapre_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_not_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_not_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_conv_float_tieaway_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_conv_float_tieaway_simd_decode\"");
    ExecuteResult::Ok
}
fn branch_unconditional_dret_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"branch_unconditional_dret_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcgettag_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcgettag_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_double_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_double_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_fp_complex_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_fp_complex_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_round_frint_32_64_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_round_frint_32_64_decode\"");
    ExecuteResult::Ok
}
fn float_arithmetic_div_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_arithmetic_div_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_cmp_fp16_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_cmp_fp16_simd_decode\"");
    ExecuteResult::Ok
}
fn memory_atomicops_cas_pair_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_atomicops_cas_pair_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_rightnarrow_uniform_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_rightnarrow_uniform_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_shift_variable_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_shift_variable_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_widen_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_widen_decode\"");
    ExecuteResult::Ok
}
fn memory_literal_simdfp_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"memory_literal_simdfp_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_mul_double_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_mul_double_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fpmax_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fpmax_sisd_decode\"");
    ExecuteResult::Ok
}
fn float_arithmetic_round_frint_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_arithmetic_round_frint_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_long_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_long_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_conv_int_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_conv_int_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode\"");
    ExecuteResult::Ok
}
fn integer_insext_extract_immediate_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_insext_extract_immediate_decode\"");
    ExecuteResult::Ok
}
fn system_exceptions_debug_exception_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_exceptions_debug_exception_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_leftinsert_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_leftinsert_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsubtag_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsubtag_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_rightnarrow_logical_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_rightnarrow_logical_decode\"");
    ExecuteResult::Ok
}
fn integer_flags_xaflag_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_flags_xaflag_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_sub_saturating_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_sub_saturating_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_halving_rounding_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_halving_rounding_decode\"");
    ExecuteResult::Ok
}
fn memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_conv_int_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_conv_int_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_long_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_long_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_address_pcrel_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_address_pcrel_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_recps_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_recps_sisd_decode\"");
    ExecuteResult::Ok
}
fn float_move_fp_imm_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_move_fp_imm_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_fp16_conv_int_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_fp16_conv_int_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_addsub_wide_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_addsub_wide_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_int_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_int_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha3_xar_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha3_xar_decode\"");
    ExecuteResult::Ok
}
fn integer_bitfield_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_bitfield_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_high_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_high_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_div_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_div_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_high_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_high_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_int_product_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_int_product_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagpairandzerodata_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagpairandzerodata_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_int_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_int_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_rev_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_rev_decode\"");
    ExecuteResult::Ok
}
fn memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fpmaxnm_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fpmaxnm_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_div_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_div_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_rsqrts_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_rsqrts_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_sqrtest_float_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_sqrtest_float_simd_decode\"");
    ExecuteResult::Ok
}
fn float_compare_uncond_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_compare_uncond_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha3op_sha1sched0_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha3op_sha1sched0_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_mul_widening_64128hi_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_mul_widening_64128hi_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_fp16_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_fp16_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_double_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_double_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcgettagarray_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcgettagarray_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_narrow_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_narrow_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_recip_int_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_recip_int_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_sqrt_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_sqrt_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_conv_float_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_conv_float_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_diffneg_sat_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_diffneg_sat_simd_decode\"");
    ExecuteResult::Ok
}
fn branch_conditional_cond_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"branch_conditional_cond_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_integer_insert_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_integer_insert_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_saturating_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_saturating_simd_decode\"");
    ExecuteResult::Ok
}
fn memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode\""
    );
    ExecuteResult::Ok
}
fn memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_maxmin_fp_1985_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_maxmin_fp_1985_decode\"");
    ExecuteResult::Ok
}
fn float_arithmetic_mul_product_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_arithmetic_mul_product_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_pacia_hint_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_pacia_hint_decode\"");
    ExecuteResult::Ok
}
fn float_convert_fp_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_convert_fp_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_clsz_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_clsz_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagpairpost_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagpairpost_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_pacib_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_pacib_dp_1src_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_fp16_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_fp16_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_vector_table_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_vector_table_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_extract_sqxtun_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_extract_sqxtun_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_rightnarrow_nonuniform_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_rightnarrow_nonuniform_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_rbit_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_rbit_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha3op_sha1hash_choose_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha3op_sha1hash_choose_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_sqrtest_fp16_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_sqrtest_fp16_simd_decode\"");
    ExecuteResult::Ok
}
fn system_exceptions_runtime_hvc_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_exceptions_runtime_hvc_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettag_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettag_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_pointer_mcsubtracttaggedaddress_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_pointer_mcsubtracttaggedaddress_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_complex_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_complex_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_mul_dmacc_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_mul_dmacc_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode\""
    );
    ExecuteResult::Ok
}
fn vector_reduce_addlong_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_addlong_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_sqrtfp16_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_sqrtfp16_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_recip_fp16_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_recip_fp16_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_maxmin_single_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_maxmin_single_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagandzerodata_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagandzerodata_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_xtn_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_xtn_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_pacia_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_pacia_dp_1src_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha3op_sha1hash_parity_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha3op_sha1hash_parity_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha2op_sha256sched0_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha2op_sha256sched0_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagarray_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagarray_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_int_lessthan_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_int_lessthan_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_flags_axflag_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_flags_axflag_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_int_accum_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_int_accum_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_float_lessthan_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_float_lessthan_sisd_decode\"");
    ExecuteResult::Ok
}
fn system_sysops_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_sysops_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mulacc_double_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mulacc_double_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_mul_poly_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_mul_poly_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_immediate_signed_pac_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"memory_single_general_immediate_signed_pac_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_add_fp_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_add_fp_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_rbit_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_rbit_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp_product_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp_product_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_leftinsert_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_leftinsert_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_recpsfp16_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_recpsfp16_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_pacib_hint_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_pacib_hint_decode\"");
    ExecuteResult::Ok
}
fn system_exceptions_debug_halt_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_exceptions_debug_halt_decode\"");
    ExecuteResult::Ok
}
fn memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_recps_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_recps_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_left_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_left_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_frecpx_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_frecpx_decode\"");
    ExecuteResult::Ok
}
fn integer_flags_cfinv_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_flags_cfinv_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fpadd_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fpadd_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcinsertrandomtag_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcinsertrandomtag_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_fp16_conv_int_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_fp16_conv_int_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_pacga_dp_2src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_pacga_dp_2src_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagpair_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagpair_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_autia_hint_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_autia_hint_decode\"");
    ExecuteResult::Ok
}
fn integer_flags_rmif_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_flags_rmif_decode\"");
    ExecuteResult::Ok
}
fn system_exceptions_runtime_svc_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_exceptions_runtime_svc_decode\"");
    ExecuteResult::Ok
}
fn branch_unconditional_eret_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"branch_unconditional_eret_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_add_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_add_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_diffneg_int_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_diffneg_int_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_sub_fp16_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_sub_fp16_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fpmaxnm_simd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fpmaxnm_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_mul_product_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_mul_product_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_fp16max_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_fp16max_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_flags_setf_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_flags_setf_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_integer_move_signed_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_integer_move_signed_decode\"");
    ExecuteResult::Ok
}
fn system_exceptions_debug_breakpoint_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_exceptions_debug_breakpoint_decode\"");
    ExecuteResult::Ok
}
fn vector_shift_rightinsert_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_shift_rightinsert_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_pacda_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_pacda_dp_1src_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_recip_float_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_recip_float_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_reduce_add_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_reduce_add_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagpost_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagpost_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_vector_insert_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_vector_insert_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_strip_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_strip_dp_1src_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_float_conv_int_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_float_conv_int_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_pacdb_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_pacdb_dp_1src_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagandzerodatapost_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagandzerodatapost_decode\"");
    ExecuteResult::Ok
}
fn integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_sub_saturating_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_sub_saturating_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_disparate_mul_double_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_disparate_mul_double_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_autib_hint_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_autib_hint_decode\"");
    ExecuteResult::Ok
}
fn float_arithmetic_round_frint_32_64_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_arithmetic_round_frint_32_64_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_high_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_high_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_vector_cpydup_sisd_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_vector_cpydup_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_crypto_sha2op_sha1sched1_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_crypto_sha2op_sha1sched1_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagpairandzerodatapre_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagpairandzerodatapre_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_sub_fp_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_sub_fp_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_strip_hint_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_strip_hint_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_shift_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_shift_decode\"");
    ExecuteResult::Ok
}
fn vector_transfer_vector_extract_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_transfer_vector_extract_decode\"");
    ExecuteResult::Ok
}
fn system_exceptions_runtime_smc_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"system_exceptions_runtime_smc_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_element_mul_double_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_element_mul_double_sisd_decode\"");
    ExecuteResult::Ok
}
fn memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!(
        "decoded \"memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode\""
    );
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_special_sqrtest_float_sisd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_special_sqrtest_float_sisd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettagpre_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettagpre_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cnt_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cnt_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_unary_cmp_float_lessthan_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_unary_cmp_float_lessthan_simd_decode\"");
    ExecuteResult::Ok
}
fn vector_arithmetic_binary_uniform_rsqrts_simd_decode(
    state: &mut AArch64CoreState,
) -> ExecuteResult {
    log::trace!("decoded \"vector_arithmetic_binary_uniform_rsqrts_simd_decode\"");
    ExecuteResult::Ok
}
fn integer_tags_mcsettaganddatapairpost_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_tags_mcsettaganddatapairpost_decode\"");
    ExecuteResult::Ok
}
fn float_move_fp_select_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"float_move_fp_select_decode\"");
    ExecuteResult::Ok
}
fn integer_pac_autia_dp_1src_decode(state: &mut AArch64CoreState) -> ExecuteResult {
    log::trace!("decoded \"integer_pac_autia_dp_1src_decode\"");
    ExecuteResult::Ok
}
