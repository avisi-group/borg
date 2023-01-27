#pragma once
#include <decode.h>
namespace captive 
{
  namespace arch 
  {
    namespace aarch64 
    {
      class aarch64_decode : public Decode
      {
        public:
        enum aarch64_isa_modes 
        {
          aarch64_aarch64 = 0,
        }
        ;
        enum aarch64_opcodes 
        {
          aarch64_aarch64_branch_conditional_compare_decode4660422,
          aarch64_aarch64_branch_conditional_compare_decode4663551,
          aarch64_aarch64_branch_conditional_cond_decode4661873,
          aarch64_aarch64_branch_conditional_test_decode4660675,
          aarch64_aarch64_branch_conditional_test_decode4661704,
          aarch64_aarch64_branch_unconditional_dret_decode4660999,
          aarch64_aarch64_branch_unconditional_eret_decode4662816,
          aarch64_aarch64_branch_unconditional_eret_decode4662893,
          aarch64_aarch64_branch_unconditional_immediate_decode4660600,
          aarch64_aarch64_branch_unconditional_immediate_decode4662335,
          aarch64_aarch64_branch_unconditional_register_decode4660339,
          aarch64_aarch64_branch_unconditional_register_decode4661183,
          aarch64_aarch64_branch_unconditional_register_decode4661988,
          aarch64_aarch64_branch_unconditional_register_decode4662511,
          aarch64_aarch64_branch_unconditional_register_decode4662554,
          aarch64_aarch64_branch_unconditional_register_decode4663102,
          aarch64_aarch64_float_arithmetic_addsub_decode4659884,
          aarch64_aarch64_float_arithmetic_addsub_decode4660164,
          aarch64_aarch64_float_arithmetic_div_decode4661050,
          aarch64_aarch64_float_arithmetic_maxmin_decode4660799,
          aarch64_aarch64_float_arithmetic_maxmin_decode4661464,
          aarch64_aarch64_float_arithmetic_maxmin_decode4662587,
          aarch64_aarch64_float_arithmetic_maxmin_decode4662741,
          aarch64_aarch64_float_arithmetic_mul_addsub_decode4659975,
          aarch64_aarch64_float_arithmetic_mul_addsub_decode4660987,
          aarch64_aarch64_float_arithmetic_mul_addsub_decode4661929,
          aarch64_aarch64_float_arithmetic_mul_addsub_decode4663122,
          aarch64_aarch64_float_arithmetic_mul_product_decode4661960,
          aarch64_aarch64_float_arithmetic_mul_product_decode4662459,
          aarch64_aarch64_float_arithmetic_round_frint_32_64_decode4663342,
          aarch64_aarch64_float_arithmetic_round_frint_decode4661146,
          aarch64_aarch64_float_arithmetic_round_frint_decode4661406,
          aarch64_aarch64_float_arithmetic_round_frint_decode4661851,
          aarch64_aarch64_float_arithmetic_round_frint_decode4662480,
          aarch64_aarch64_float_arithmetic_round_frint_decode4662831,
          aarch64_aarch64_float_arithmetic_round_frint_decode4662913,
          aarch64_aarch64_float_arithmetic_round_frint_decode4663447,
          aarch64_aarch64_float_arithmetic_unary_decode4660079,
          aarch64_aarch64_float_arithmetic_unary_decode4660095,
          aarch64_aarch64_float_arithmetic_unary_decode4661230,
          aarch64_aarch64_float_arithmetic_unary_decode4662972,
          aarch64_aarch64_float_compare_cond_decode4660204,
          aarch64_aarch64_float_compare_cond_decode4662650,
          aarch64_aarch64_float_compare_uncond_decode4661696,
          aarch64_aarch64_float_compare_uncond_decode4661978,
          aarch64_aarch64_float_convert_fix_decode4659868,
          aarch64_aarch64_float_convert_fix_decode4660589,
          aarch64_aarch64_float_convert_fix_decode4660609,
          aarch64_aarch64_float_convert_fix_decode4663376,
          aarch64_aarch64_float_convert_fp_decode4661993,
          aarch64_aarch64_float_convert_int_decode4660814,
          aarch64_aarch64_float_convert_int_decode4660937,
          aarch64_aarch64_float_convert_int_decode4661166,
          aarch64_aarch64_float_convert_int_decode4661592,
          aarch64_aarch64_float_convert_int_decode4661815,
          aarch64_aarch64_float_convert_int_decode4661967,
          aarch64_aarch64_float_convert_int_decode4662018,
          aarch64_aarch64_float_convert_int_decode4662409,
          aarch64_aarch64_float_convert_int_decode4662431,
          aarch64_aarch64_float_convert_int_decode4662930,
          aarch64_aarch64_float_convert_int_decode4662989,
          aarch64_aarch64_float_convert_int_decode4663005,
          aarch64_aarch64_float_convert_int_decode4663285,
          aarch64_aarch64_float_convert_int_decode4663836,
          aarch64_aarch64_float_move_fp_imm_decode4661440,
          aarch64_aarch64_float_move_fp_select_decode4663824,
          aarch64_aarch64_integer_arithmetic_address_pcrel_decode4661401,
          aarch64_aarch64_integer_arithmetic_address_pcrel_decode4662813,
          aarch64_aarch64_integer_arithmetic_addsub_carry_decode4659891,
          aarch64_aarch64_integer_arithmetic_addsub_carry_decode4660241,
          aarch64_aarch64_integer_arithmetic_addsub_carry_decode4660357,
          aarch64_aarch64_integer_arithmetic_addsub_carry_decode4661190,
          aarch64_aarch64_integer_arithmetic_addsub_extendedreg_decode4659985,
          aarch64_aarch64_integer_arithmetic_addsub_extendedreg_decode4660040,
          aarch64_aarch64_integer_arithmetic_addsub_extendedreg_decode4661387,
          aarch64_aarch64_integer_arithmetic_addsub_extendedreg_decode4663497,
          aarch64_aarch64_integer_arithmetic_addsub_immediate_decode4659928,
          aarch64_aarch64_integer_arithmetic_addsub_immediate_decode4660472,
          aarch64_aarch64_integer_arithmetic_addsub_immediate_decode4660774,
          aarch64_aarch64_integer_arithmetic_addsub_immediate_decode4661399,
          aarch64_aarch64_integer_arithmetic_addsub_shiftedreg_decode4659762,
          aarch64_aarch64_integer_arithmetic_addsub_shiftedreg_decode4660207,
          aarch64_aarch64_integer_arithmetic_addsub_shiftedreg_decode4662735,
          aarch64_aarch64_integer_arithmetic_addsub_shiftedreg_decode4663364,
          aarch64_aarch64_integer_arithmetic_cnt_decode4660660,
          aarch64_aarch64_integer_arithmetic_cnt_decode4663297,
          aarch64_aarch64_integer_arithmetic_div_decode4661638,
          aarch64_aarch64_integer_arithmetic_div_decode4663717,
          aarch64_aarch64_integer_arithmetic_mul_uniform_addsub_decode4660051,
          aarch64_aarch64_integer_arithmetic_mul_uniform_addsub_decode4661006,
          aarch64_aarch64_integer_arithmetic_mul_widening_3264_decode4659759,
          aarch64_aarch64_integer_arithmetic_mul_widening_3264_decode4660425,
          aarch64_aarch64_integer_arithmetic_mul_widening_3264_decode4660673,
          aarch64_aarch64_integer_arithmetic_mul_widening_3264_decode4662921,
          aarch64_aarch64_integer_arithmetic_mul_widening_64128hi_decode4661763,
          aarch64_aarch64_integer_arithmetic_mul_widening_64128hi_decode4663750,
          aarch64_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164,
          aarch64_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191,
          aarch64_aarch64_integer_arithmetic_rbit_decode4662535,
          aarch64_aarch64_integer_arithmetic_rev_decode4661605,
          aarch64_aarch64_integer_arithmetic_rev_decode4663076,
          aarch64_aarch64_integer_arithmetic_rev_decode4663238,
          aarch64_aarch64_integer_bitfield_decode4661491,
          aarch64_aarch64_integer_bitfield_decode4662760,
          aarch64_aarch64_integer_bitfield_decode4663093,
          aarch64_aarch64_integer_conditional_compare_immediate_decode4660012,
          aarch64_aarch64_integer_conditional_compare_immediate_decode4662419,
          aarch64_aarch64_integer_conditional_compare_register_decode4660228,
          aarch64_aarch64_integer_conditional_compare_register_decode4662874,
          aarch64_aarch64_integer_conditional_select_decode4660236,
          aarch64_aarch64_integer_conditional_select_decode4661582,
          aarch64_aarch64_integer_conditional_select_decode4661680,
          aarch64_aarch64_integer_conditional_select_decode4662744,
          aarch64_aarch64_integer_crc_decode4660733,
          aarch64_aarch64_integer_crc_decode4662762,
          aarch64_aarch64_integer_flags_axflag_decode4662393,
          aarch64_aarch64_integer_flags_cfinv_decode4662708,
          aarch64_aarch64_integer_flags_rmif_decode4662797,
          aarch64_aarch64_integer_flags_setf_decode4662979,
          aarch64_aarch64_integer_flags_xaflag_decode4661273,
          aarch64_aarch64_integer_insext_extract_immediate_decode4661192,
          aarch64_aarch64_integer_insext_insert_movewide_decode4660386,
          aarch64_aarch64_integer_insext_insert_movewide_decode4661817,
          aarch64_aarch64_integer_insext_insert_movewide_decode4662070,
          aarch64_aarch64_integer_logical_immediate_decode4660553,
          aarch64_aarch64_integer_logical_immediate_decode4662254,
          aarch64_aarch64_integer_logical_immediate_decode4662411,
          aarch64_aarch64_integer_logical_immediate_decode4662603,
          aarch64_aarch64_integer_logical_shiftedreg_decode4660725,
          aarch64_aarch64_integer_logical_shiftedreg_decode4660895,
          aarch64_aarch64_integer_logical_shiftedreg_decode4660912,
          aarch64_aarch64_integer_logical_shiftedreg_decode4662645,
          aarch64_aarch64_integer_logical_shiftedreg_decode4663175,
          aarch64_aarch64_integer_logical_shiftedreg_decode4663421,
          aarch64_aarch64_integer_logical_shiftedreg_decode4663481,
          aarch64_aarch64_integer_logical_shiftedreg_decode4663672,
          aarch64_aarch64_integer_pac_autda_dp_1src_decode4660872,
          aarch64_aarch64_integer_pac_autdb_dp_1src_decode4660269,
          aarch64_aarch64_integer_pac_autia_dp_1src_decode4663841,
          aarch64_aarch64_integer_pac_autia_hint_decode4662773,
          aarch64_aarch64_integer_pac_autib_dp_1src_decode4659802,
          aarch64_aarch64_integer_pac_autib_hint_decode4663304,
          aarch64_aarch64_integer_pac_pacda_dp_1src_decode4663015,
          aarch64_aarch64_integer_pac_pacdb_dp_1src_decode4663160,
          aarch64_aarch64_integer_pac_pacga_dp_2src_decode4662751,
          aarch64_aarch64_integer_pac_pacia_dp_1src_decode4662350,
          aarch64_aarch64_integer_pac_pacia_hint_decode4661962,
          aarch64_aarch64_integer_pac_pacib_dp_1src_decode4662013,
          aarch64_aarch64_integer_pac_pacib_hint_decode4662575,
          aarch64_aarch64_integer_pac_strip_dp_1src_decode4663140,
          aarch64_aarch64_integer_pac_strip_hint_decode4663565,
          aarch64_aarch64_integer_shift_variable_decode4661093,
          aarch64_aarch64_integer_shift_variable_decode4661771,
          aarch64_aarch64_integer_shift_variable_decode4663366,
          aarch64_aarch64_integer_shift_variable_decode4663826,
          aarch64_aarch64_integer_tags_mcaddtag_decode4659982,
          aarch64_aarch64_integer_tags_mcgettag_decode4661013,
          aarch64_aarch64_integer_tags_mcgettagarray_decode4661787,
          aarch64_aarch64_integer_tags_mcinsertrandomtag_decode4662727,
          aarch64_aarch64_integer_tags_mcinserttagmask_decode4659904,
          aarch64_aarch64_integer_tags_mcsettag_decode4662156,
          aarch64_aarch64_integer_tags_mcsettaganddatapair_decode4659797,
          aarch64_aarch64_integer_tags_mcsettaganddatapairpost_decode4663819,
          aarch64_aarch64_integer_tags_mcsettaganddatapairpre_decode4660925,
          aarch64_aarch64_integer_tags_mcsettagandzerodata_decode4662281,
          aarch64_aarch64_integer_tags_mcsettagandzerodatapost_decode4663163,
          aarch64_aarch64_integer_tags_mcsettagandzerodatapre_decode4660974,
          aarch64_aarch64_integer_tags_mcsettagarray_decode4662372,
          aarch64_aarch64_integer_tags_mcsettagpair_decode4662758,
          aarch64_aarch64_integer_tags_mcsettagpairandzerodata_decode4661595,
          aarch64_aarch64_integer_tags_mcsettagpairandzerodatapost_decode4660126,
          aarch64_aarch64_integer_tags_mcsettagpairandzerodatapre_decode4663450,
          aarch64_aarch64_integer_tags_mcsettagpairpost_decode4662003,
          aarch64_aarch64_integer_tags_mcsettagpairpre_decode4660116,
          aarch64_aarch64_integer_tags_mcsettagpost_decode4663072,
          aarch64_aarch64_integer_tags_mcsettagpre_decode4663726,
          aarch64_aarch64_integer_tags_mcsubtag_decode4661222,
          aarch64_aarch64_memory_atomicops_cas_pair_decode4661064,
          aarch64_aarch64_memory_atomicops_cas_single_decode4660193,
          aarch64_aarch64_memory_atomicops_cas_single_decode4662333,
          aarch64_aarch64_memory_atomicops_cas_single_decode4663354,
          aarch64_aarch64_memory_atomicops_ld_decode4659787,
          aarch64_aarch64_memory_atomicops_ld_decode4659842,
          aarch64_aarch64_memory_atomicops_ld_decode4659856,
          aarch64_aarch64_memory_atomicops_ld_decode4659889,
          aarch64_aarch64_memory_atomicops_ld_decode4659972,
          aarch64_aarch64_memory_atomicops_ld_decode4660233,
          aarch64_aarch64_memory_atomicops_ld_decode4660279,
          aarch64_aarch64_memory_atomicops_ld_decode4660829,
          aarch64_aarch64_memory_atomicops_ld_decode4661270,
          aarch64_aarch64_memory_atomicops_ld_decode4661392,
          aarch64_aarch64_memory_atomicops_ld_decode4661785,
          aarch64_aarch64_memory_atomicops_ld_decode4662136,
          aarch64_aarch64_memory_atomicops_ld_decode4662278,
          aarch64_aarch64_memory_atomicops_ld_decode4662618,
          aarch64_aarch64_memory_atomicops_ld_decode4662655,
          aarch64_aarch64_memory_atomicops_ld_decode4662804,
          aarch64_aarch64_memory_atomicops_ld_decode4662846,
          aarch64_aarch64_memory_atomicops_ld_decode4662879,
          aarch64_aarch64_memory_atomicops_ld_decode4663269,
          aarch64_aarch64_memory_atomicops_ld_decode4663302,
          aarch64_aarch64_memory_atomicops_ld_decode4663401,
          aarch64_aarch64_memory_atomicops_ld_decode4663632,
          aarch64_aarch64_memory_atomicops_ld_decode4663637,
          aarch64_aarch64_memory_atomicops_ld_decode4663846,
          aarch64_aarch64_memory_atomicops_st_decode4659773,
          aarch64_aarch64_memory_atomicops_st_decode4659926,
          aarch64_aarch64_memory_atomicops_st_decode4659967,
          aarch64_aarch64_memory_atomicops_st_decode4660032,
          aarch64_aarch64_memory_atomicops_st_decode4660362,
          aarch64_aarch64_memory_atomicops_st_decode4660402,
          aarch64_aarch64_memory_atomicops_st_decode4660527,
          aarch64_aarch64_memory_atomicops_st_decode4660649,
          aarch64_aarch64_memory_atomicops_st_decode4661098,
          aarch64_aarch64_memory_atomicops_st_decode4661130,
          aarch64_aarch64_memory_atomicops_st_decode4661151,
          aarch64_aarch64_memory_atomicops_st_decode4661241,
          aarch64_aarch64_memory_atomicops_st_decode4661326,
          aarch64_aarch64_memory_atomicops_st_decode4661555,
          aarch64_aarch64_memory_atomicops_st_decode4661654,
          aarch64_aarch64_memory_atomicops_st_decode4661690,
          aarch64_aarch64_memory_atomicops_st_decode4661709,
          aarch64_aarch64_memory_atomicops_st_decode4661726,
          aarch64_aarch64_memory_atomicops_st_decode4661840,
          aarch64_aarch64_memory_atomicops_st_decode4661856,
          aarch64_aarch64_memory_atomicops_st_decode4662436,
          aarch64_aarch64_memory_atomicops_st_decode4663086,
          aarch64_aarch64_memory_atomicops_st_decode4663470,
          aarch64_aarch64_memory_atomicops_st_decode4663768,
          aarch64_aarch64_memory_atomicops_swp_decode4660004,
          aarch64_aarch64_memory_atomicops_swp_decode4662192,
          aarch64_aarch64_memory_atomicops_swp_decode4662871,
          aarch64_aarch64_memory_exclusive_pair_decode4660282,
          aarch64_aarch64_memory_exclusive_pair_decode4660708,
          aarch64_aarch64_memory_exclusive_pair_decode4660763,
          aarch64_aarch64_memory_exclusive_pair_decode4662692,
          aarch64_aarch64_memory_exclusive_single_decode4659962,
          aarch64_aarch64_memory_exclusive_single_decode4660118,
          aarch64_aarch64_memory_exclusive_single_decode4660782,
          aarch64_aarch64_memory_exclusive_single_decode4660927,
          aarch64_aarch64_memory_exclusive_single_decode4661421,
          aarch64_aarch64_memory_exclusive_single_decode4661799,
          aarch64_aarch64_memory_exclusive_single_decode4661995,
          aarch64_aarch64_memory_exclusive_single_decode4662818,
          aarch64_aarch64_memory_exclusive_single_decode4663044,
          aarch64_aarch64_memory_exclusive_single_decode4663108,
          aarch64_aarch64_memory_exclusive_single_decode4663386,
          aarch64_aarch64_memory_exclusive_single_decode4663615,
          aarch64_aarch64_memory_literal_general_decode4660239,
          aarch64_aarch64_memory_literal_general_decode4660756,
          aarch64_aarch64_memory_literal_general_decode4663539,
          aarch64_aarch64_memory_literal_simdfp_decode4661110,
          aarch64_aarch64_memory_ordered_decode4659830,
          aarch64_aarch64_memory_ordered_decode4659911,
          aarch64_aarch64_memory_ordered_decode4660053,
          aarch64_aarch64_memory_ordered_decode4660611,
          aarch64_aarch64_memory_ordered_decode4661081,
          aarch64_aarch64_memory_ordered_decode4662032,
          aarch64_aarch64_memory_ordered_decode4662426,
          aarch64_aarch64_memory_ordered_decode4663182,
          aarch64_aarch64_memory_ordered_decode4663316,
          aarch64_aarch64_memory_ordered_decode4663361,
          aarch64_aarch64_memory_ordered_decode4663419,
          aarch64_aarch64_memory_ordered_decode4663670,
          aarch64_aarch64_memory_orderedrcpc_decode4660923,
          aarch64_aarch64_memory_orderedrcpc_decode4662110,
          aarch64_aarch64_memory_orderedrcpc_decode4662823,
          aarch64_aarch64_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572,
          aarch64_aarch64_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826,
          aarch64_aarch64_memory_pair_general_offset_memory_pair_general_postidx__decode4659909,
          aarch64_aarch64_memory_pair_general_offset_memory_pair_general_postidx__decode4661225,
          aarch64_aarch64_memory_pair_general_offset_memory_pair_general_postidx__decode4663715,
          aarch64_aarch64_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481,
          aarch64_aarch64_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204,
          aarch64_aarch64_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784,
          aarch64_aarch64_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782,
          aarch64_aarch64_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265,
          aarch64_aarch64_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621,
          aarch64_aarch64_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915,
          aarch64_aarch64_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706,
          aarch64_aarch64_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951,
          aarch64_aarch64_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580,
          aarch64_aarch64_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613,
          aarch64_aarch64_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267,
          aarch64_aarch64_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639,
          aarch64_aarch64_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274,
          aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622,
          aarch64_aarch64_memory_single_general_immediate_signed_pac_decode4662489,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347,
          aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718,
          aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797,
          aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4660430,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4660435,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4661209,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4661643,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4661920,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4662851,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4663396,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4663570,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4663642,
          aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4663747,
          aarch64_aarch64_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252,
          aarch64_aarch64_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207,
          aarch64_aarch64_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940,
          aarch64_aarch64_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940,
          aarch64_aarch64_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613,
          aarch64_aarch64_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145,
          aarch64_aarch64_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315,
          aarch64_aarch64_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703,
          aarch64_aarch64_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446,
          aarch64_aarch64_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887,
          aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551,
          aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022,
          aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425,
          aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630,
          aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386,
          aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484,
          aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167,
          aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650,
          aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999,
          aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343,
          aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858,
          aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449,
          aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891,
          aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493,
          aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106,
          aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925,
          aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506,
          aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776,
          aarch64_aarch64_system_barriers_decode4659845,
          aarch64_aarch64_system_barriers_decode4660260,
          aarch64_aarch64_system_barriers_decode4660832,
          aarch64_aarch64_system_barriers_decode4663379,
          aarch64_aarch64_system_exceptions_debug_breakpoint_decode4662991,
          aarch64_aarch64_system_exceptions_debug_exception_decode4661211,
          aarch64_aarch64_system_exceptions_debug_exception_decode4662084,
          aarch64_aarch64_system_exceptions_debug_exception_decode4663287,
          aarch64_aarch64_system_exceptions_debug_halt_decode4662594,
          aarch64_aarch64_system_exceptions_runtime_hvc_decode4662148,
          aarch64_aarch64_system_exceptions_runtime_smc_decode4663617,
          aarch64_aarch64_system_exceptions_runtime_svc_decode4662799,
          aarch64_aarch64_system_hints_decode4659837,
          aarch64_aarch64_system_hints_decode4659906,
          aarch64_aarch64_system_hints_decode4660103,
          aarch64_aarch64_system_hints_decode4660651,
          aarch64_aarch64_system_hints_decode4661636,
          aarch64_aarch64_system_hints_decode4662327,
          aarch64_aarch64_system_hints_decode4662438,
          aarch64_aarch64_system_hints_decode4662596,
          aarch64_aarch64_system_hints_decode4663225,
          aarch64_aarch64_system_monitors_decode4660195,
          aarch64_aarch64_system_register_cpsr_decode4660345,
          aarch64_aarch64_system_register_system_decode4660215,
          aarch64_aarch64_system_register_system_decode4663228,
          aarch64_aarch64_system_sysops_decode4662440,
          aarch64_aarch64_system_sysops_decode4662507,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_long_decode4660687,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_long_decode4661384,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_long_decode4661481,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_long_decode4661538,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_wide_decode4661455,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_wide_decode4662052,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_wide_decode4662076,
          aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_wide_decode4662304,
          aarch64_aarch64_vector_arithmetic_binary_disparate_diff_decode4659934,
          aarch64_aarch64_vector_arithmetic_binary_disparate_diff_decode4660959,
          aarch64_aarch64_vector_arithmetic_binary_disparate_diff_decode4661040,
          aarch64_aarch64_vector_arithmetic_binary_disparate_diff_decode4661866,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_accum_decode4660838,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_accum_decode4661157,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_accum_decode4662665,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_accum_decode4663433,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_poly_decode4662475,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_product_decode4662951,
          aarch64_aarch64_vector_arithmetic_binary_disparate_mul_product_decode4663511,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_double_simd_decode4661780,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_double_sisd_decode4663668,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_fp_simd_decode4660225,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_fp_simd_decode4662030,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_high_simd_decode4661532,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_high_simd_decode4661804,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_high_sisd_decode4663391,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_high_sisd_decode4663831,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_int_decode4661468,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_long_decode4661161,
          aarch64_aarch64_vector_arithmetic_binary_element_mul_long_decode4663607,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_complex_decode4662170,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_double_simd_decode4662946,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661028,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_int_decode4661601,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_int_decode4662684,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_long_decode4661367,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_long_decode4662038,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_long_decode4662581,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_long_decode4662885,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926,
          aarch64_aarch64_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp16_decode4660854,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp16_decode4661141,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp_decode4662516,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp_decode4663119,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_decode4661312,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_decode4659828,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_halving_truncating_decode4663677,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_decode4661901,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_saturating_simd_decode4661972,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382,
          aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935,
          aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372,
          aarch64_aarch64_vector_arithmetic_binary_uniform_diff_decode4660918,
          aarch64_aarch64_vector_arithmetic_binary_uniform_diff_decode4660953,
          aarch64_aarch64_vector_arithmetic_binary_uniform_diff_decode4662320,
          aarch64_aarch64_vector_arithmetic_binary_uniform_diff_decode4663026,
          aarch64_aarch64_vector_arithmetic_binary_uniform_div_decode4661518,
          aarch64_aarch64_vector_arithmetic_binary_uniform_divfp16_decode4660971,
          aarch64_aarch64_vector_arithmetic_binary_uniform_logical_andorr_decode4659823,
          aarch64_aarch64_vector_arithmetic_binary_uniform_logical_andorr_decode4661345,
          aarch64_aarch64_vector_arithmetic_binary_uniform_logical_andorr_decode4663314,
          aarch64_aarch64_vector_arithmetic_binary_uniform_logical_andorr_decode4663460,
          aarch64_aarch64_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465,
          aarch64_aarch64_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776,
          aarch64_aarch64_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197,
          aarch64_aarch64_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4661945,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4663049,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_single_decode4662273,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_single_decode4662399,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_single_decode4662724,
          aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_single_decode4663064,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode4662057,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode4663352,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_product_decode4661569,
          aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_product_decode4662314,
          aarch64_aarch64_vector_arithmetic_binary_uniform_recps_simd_decode4662639,
          aarch64_aarch64_vector_arithmetic_binary_uniform_recps_sisd_decode4661430,
          aarch64_aarch64_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506,
          aarch64_aarch64_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568,
          aarch64_aarch64_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794,
          aarch64_aarch64_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672,
          aarch64_aarch64_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335,
          aarch64_aarch64_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_simd_decode4660420,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_simd_decode4662287,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_simd_decode4662526,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_simd_decode4663042,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_simd_decode4663173,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_simd_decode4663202,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_simd_decode4663259,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_simd_decode4663723,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4660309,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4660780,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4660793,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4661201,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4661257,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4661912,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4662505,
          aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4663213,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_int_decode4660804,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_int_decode4662778,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_decode4661288,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_saturating_simd_decode4661331,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196,
          aarch64_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329,
          aarch64_aarch64_vector_arithmetic_unary_add_pairwise_decode4660090,
          aarch64_aarch64_vector_arithmetic_unary_add_pairwise_decode4661018,
          aarch64_aarch64_vector_arithmetic_unary_add_pairwise_decode4661475,
          aarch64_aarch64_vector_arithmetic_unary_add_pairwise_decode4662243,
          aarch64_aarch64_vector_arithmetic_unary_add_saturating_simd_decode4660656,
          aarch64_aarch64_vector_arithmetic_unary_add_saturating_simd_decode4662464,
          aarch64_aarch64_vector_arithmetic_unary_add_saturating_sisd_decode4660460,
          aarch64_aarch64_vector_arithmetic_unary_add_saturating_sisd_decode4662809,
          aarch64_aarch64_vector_arithmetic_unary_clsz_decode4662000,
          aarch64_aarch64_vector_arithmetic_unary_clsz_decode4663218,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781,
          aarch64_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4660500,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662788,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662793,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662841,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705,
          aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode4663627,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538,
          aarch64_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377,
          aarch64_aarch64_vector_arithmetic_unary_cnt_decode4663731,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_float_decode4659939,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_float_decode4663755,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_fp16_decode4660470,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_fp16_decode4661115,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_int_simd_decode4659980,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_int_simd_decode4660274,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_int_sisd_decode4662856,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_int_sisd_decode4663223,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_sat_simd_decode4661871,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_sat_simd_decode4662592,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355,
          aarch64_aarch64_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626,
          aarch64_aarch64_vector_arithmetic_unary_extract_nosat_decode4660843,
          aarch64_aarch64_vector_arithmetic_unary_extract_sat_simd_decode4660257,
          aarch64_aarch64_vector_arithmetic_unary_extract_sat_simd_decode4662161,
          aarch64_aarch64_vector_arithmetic_unary_extract_sat_sisd_decode4660942,
          aarch64_aarch64_vector_arithmetic_unary_extract_sat_sisd_decode4663207,
          aarch64_aarch64_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105,
          aarch64_aarch64_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_int_simd_decode4663150,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_int_simd_decode4663155,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_int_sisd_decode4661340,
          aarch64_aarch64_vector_arithmetic_unary_float_conv_int_sisd_decode4661822,
          aarch64_aarch64_vector_arithmetic_unary_float_narrow_decode4661792,
          aarch64_aarch64_vector_arithmetic_unary_float_round_decode4660314,
          aarch64_aarch64_vector_arithmetic_unary_float_round_decode4660577,
          aarch64_aarch64_vector_arithmetic_unary_float_round_decode4661283,
          aarch64_aarch64_vector_arithmetic_unary_float_round_decode4662228,
          aarch64_aarch64_vector_arithmetic_unary_float_round_decode4662675,
          aarch64_aarch64_vector_arithmetic_unary_float_round_decode4663556,
          aarch64_aarch64_vector_arithmetic_unary_float_round_decode4663586,
          aarch64_aarch64_vector_arithmetic_unary_float_round_frint_32_64_decode4661045,
          aarch64_aarch64_vector_arithmetic_unary_float_widen_decode4661107,
          aarch64_aarch64_vector_arithmetic_unary_float_xtn_simd_decode4662345,
          aarch64_aarch64_vector_arithmetic_unary_float_xtn_sisd_decode4660350,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4660074,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661361,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661397,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662175,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662531,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663264,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663309,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663807,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4660113,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4662689,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661445,
          aarch64_aarch64_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661667,
          aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4659878,
          aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4660824,
          aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4661251,
          aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4661757,
          aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4662325,
          aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4663091,
          aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4663248,
          aarch64_aarch64_vector_arithmetic_unary_not_decode4660992,
          aarch64_aarch64_vector_arithmetic_unary_rbit_decode4662126,
          aarch64_aarch64_vector_arithmetic_unary_rev_decode4660516,
          aarch64_aarch64_vector_arithmetic_unary_rev_decode4660558,
          aarch64_aarch64_vector_arithmetic_unary_rev_decode4663526,
          aarch64_aarch64_vector_arithmetic_unary_shift_decode4663581,
          aarch64_aarch64_vector_arithmetic_unary_special_frecpx_decode4662670,
          aarch64_aarch64_vector_arithmetic_unary_special_frecpxfp16_decode4660511,
          aarch64_aarch64_vector_arithmetic_unary_special_recip_float_simd_decode4663020,
          aarch64_aarch64_vector_arithmetic_unary_special_recip_float_sisd_decode4660787,
          aarch64_aarch64_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252,
          aarch64_aarch64_vector_arithmetic_unary_special_recip_fp16_sisd_decode4660212,
          aarch64_aarch64_vector_arithmetic_unary_special_recip_int_decode4661797,
          aarch64_aarch64_vector_arithmetic_unary_special_sqrt_decode4661827,
          aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677,
          aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685,
          aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146,
          aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode4660910,
          aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_int_decode4660547,
          aarch64_aarch64_vector_arithmetic_unary_special_sqrtfp16_decode4662238,
          aarch64_aarch64_vector_crypto_aes_mix_decode4660264,
          aarch64_aarch64_vector_crypto_aes_mix_decode4662866,
          aarch64_aarch64_vector_crypto_aes_round_decode4660495,
          aarch64_aarch64_vector_crypto_aes_round_decode4661335,
          aarch64_aarch64_vector_crypto_sha2op_sha1hash_decode4660542,
          aarch64_aarch64_vector_crypto_sha2op_sha1sched1_decode4663437,
          aarch64_aarch64_vector_crypto_sha2op_sha256sched0_decode4662364,
          aarch64_aarch64_vector_crypto_sha3_xar_decode4661470,
          aarch64_aarch64_vector_crypto_sha3_xar_decode4662811,
          aarch64_aarch64_vector_crypto_sha3op_sha1hash_choose_decode4662141,
          aarch64_aarch64_vector_crypto_sha3op_sha1hash_majority_decode4660158,
          aarch64_aarch64_vector_crypto_sha3op_sha1hash_parity_decode4662360,
          aarch64_aarch64_vector_crypto_sha3op_sha1sched0_decode4661735,
          aarch64_aarch64_vector_crypto_sha3op_sha256hash_decode4660169,
          aarch64_aarch64_vector_crypto_sha3op_sha256hash_decode4660665,
          aarch64_aarch64_vector_crypto_sha3op_sha256sched1_decode4660329,
          aarch64_aarch64_vector_fp16_movi_decode4659795,
          aarch64_aarch64_vector_logical_decode4659791,
          aarch64_aarch64_vector_logical_decode4660044,
          aarch64_aarch64_vector_logical_decode4662755,
          aarch64_aarch64_vector_logical_decode4662766,
          aarch64_aarch64_vector_logical_decode4663664,
          aarch64_aarch64_vector_reduce_add_simd_decode4662836,
          aarch64_aarch64_vector_reduce_add_sisd_decode4663031,
          aarch64_aarch64_vector_reduce_addlong_decode4662233,
          aarch64_aarch64_vector_reduce_addlong_decode4663817,
          aarch64_aarch64_vector_reduce_fp16add_sisd_decode4660058,
          aarch64_aarch64_vector_reduce_fp16max_simd_decode4660037,
          aarch64_aarch64_vector_reduce_fp16max_simd_decode4661074,
          aarch64_aarch64_vector_reduce_fp16max_sisd_decode4662961,
          aarch64_aarch64_vector_reduce_fp16max_sisd_decode4663455,
          aarch64_aarch64_vector_reduce_fp16maxnm_simd_decode4660108,
          aarch64_aarch64_vector_reduce_fp16maxnm_simd_decode4662749,
          aarch64_aarch64_vector_reduce_fp16maxnm_sisd_decode4659916,
          aarch64_aarch64_vector_reduce_fp16maxnm_sisd_decode4663136,
          aarch64_aarch64_vector_reduce_fpadd_sisd_decode4662713,
          aarch64_aarch64_vector_reduce_fpmax_simd_decode4660185,
          aarch64_aarch64_vector_reduce_fpmax_simd_decode4662309,
          aarch64_aarch64_vector_reduce_fpmax_sisd_decode4661135,
          aarch64_aarch64_vector_reduce_fpmax_sisd_decode4661906,
          aarch64_aarch64_vector_reduce_fpmaxnm_simd_decode4662935,
          aarch64_aarch64_vector_reduce_fpmaxnm_simd_decode4663384,
          aarch64_aarch64_vector_reduce_fpmaxnm_sisd_decode4661626,
          aarch64_aarch64_vector_reduce_fpmaxnm_sisd_decode4663544,
          aarch64_aarch64_vector_reduce_intmax_decode4660141,
          aarch64_aarch64_vector_reduce_intmax_decode4660298,
          aarch64_aarch64_vector_reduce_intmax_decode4661618,
          aarch64_aarch64_vector_reduce_intmax_decode4662918,
          aarch64_aarch64_vector_shift_conv_float_simd_decode4661860,
          aarch64_aarch64_vector_shift_conv_float_simd_decode4663763,
          aarch64_aarch64_vector_shift_conv_float_sisd_decode4660737,
          aarch64_aarch64_vector_shift_conv_float_sisd_decode4662390,
          aarch64_aarch64_vector_shift_conv_int_simd_decode4661170,
          aarch64_aarch64_vector_shift_conv_int_simd_decode4662247,
          aarch64_aarch64_vector_shift_conv_int_sisd_decode4659995,
          aarch64_aarch64_vector_shift_conv_int_sisd_decode4660199,
          aarch64_aarch64_vector_shift_left_simd_decode4660772,
          aarch64_aarch64_vector_shift_left_sisd_decode4662659,
          aarch64_aarch64_vector_shift_leftinsert_simd_decode4662558,
          aarch64_aarch64_vector_shift_leftinsert_sisd_decode4661220,
          aarch64_aarch64_vector_shift_leftlong_decode4660621,
          aarch64_aarch64_vector_shift_leftlong_decode4661102,
          aarch64_aarch64_vector_shift_leftsat_simd_decode4660748,
          aarch64_aarch64_vector_shift_leftsat_simd_decode4661587,
          aarch64_aarch64_vector_shift_leftsat_simd_decode4661882,
          aarch64_aarch64_vector_shift_leftsat_sisd_decode4660585,
          aarch64_aarch64_vector_shift_leftsat_sisd_decode4663536,
          aarch64_aarch64_vector_shift_leftsat_sisd_decode4663736,
          aarch64_aarch64_vector_shift_right_simd_decode4660863,
          aarch64_aarch64_vector_shift_right_simd_decode4660888,
          aarch64_aarch64_vector_shift_right_simd_decode4661496,
          aarch64_aarch64_vector_shift_right_simd_decode4661574,
          aarch64_aarch64_vector_shift_right_simd_decode4662100,
          aarch64_aarch64_vector_shift_right_simd_decode4662298,
          aarch64_aarch64_vector_shift_right_simd_decode4663494,
          aarch64_aarch64_vector_shift_right_simd_decode4663531,
          aarch64_aarch64_vector_shift_right_sisd_decode4660009,
          aarch64_aarch64_vector_shift_right_sisd_decode4660893,
          aarch64_aarch64_vector_shift_right_sisd_decode4660984,
          aarch64_aarch64_vector_shift_right_sisd_decode4661579,
          aarch64_aarch64_vector_shift_right_sisd_decode4662095,
          aarch64_aarch64_vector_shift_right_sisd_decode4662908,
          aarch64_aarch64_vector_shift_right_sisd_decode4663597,
          aarch64_aarch64_vector_shift_right_sisd_decode4663695,
          aarch64_aarch64_vector_shift_rightinsert_simd_decode4660867,
          aarch64_aarch64_vector_shift_rightinsert_sisd_decode4662995,
          aarch64_aarch64_vector_shift_rightnarrow_logical_decode4661246,
          aarch64_aarch64_vector_shift_rightnarrow_logical_decode4663010,
          aarch64_aarch64_vector_shift_rightnarrow_nonuniform_simd_decode4662115,
          aarch64_aarch64_vector_shift_rightnarrow_nonuniform_simd_decode4662608,
          aarch64_aarch64_vector_shift_rightnarrow_nonuniform_sisd_decode4660700,
          aarch64_aarch64_vector_shift_rightnarrow_nonuniform_sisd_decode4660979,
          aarch64_aarch64_vector_shift_rightnarrow_uniform_simd_decode4660319,
          aarch64_aarch64_vector_shift_rightnarrow_uniform_simd_decode4662217,
          aarch64_aarch64_vector_shift_rightnarrow_uniform_simd_decode4662628,
          aarch64_aarch64_vector_shift_rightnarrow_uniform_simd_decode4663802,
          aarch64_aarch64_vector_shift_rightnarrow_uniform_sisd_decode4661069,
          aarch64_aarch64_vector_shift_rightnarrow_uniform_sisd_decode4661527,
          aarch64_aarch64_vector_shift_rightnarrow_uniform_sisd_decode4662450,
          aarch64_aarch64_vector_shift_rightnarrow_uniform_sisd_decode4662573,
          aarch64_aarch64_vector_transfer_integer_dup_decode4659921,
          aarch64_aarch64_vector_transfer_integer_insert_decode4661896,
          aarch64_aarch64_vector_transfer_integer_move_signed_decode4662984,
          aarch64_aarch64_vector_transfer_integer_move_unsigned_decode4660293,
          aarch64_aarch64_vector_transfer_vector_cpydup_simd_decode4660598,
          aarch64_aarch64_vector_transfer_vector_cpydup_sisd_decode4663417,
          aarch64_aarch64_vector_transfer_vector_extract_decode4663603,
          aarch64_aarch64_vector_transfer_vector_insert_decode4663098,
          aarch64_aarch64_vector_transfer_vector_permute_transpose_decode4660681,
          aarch64_aarch64_vector_transfer_vector_permute_transpose_decode4661702,
          aarch64_aarch64_vector_transfer_vector_permute_unzip_decode4660564,
          aarch64_aarch64_vector_transfer_vector_permute_unzip_decode4661752,
          aarch64_aarch64_vector_transfer_vector_permute_zip_decode4660731,
          aarch64_aarch64_vector_transfer_vector_permute_zip_decode4661321,
          aarch64_aarch64_vector_transfer_vector_table_decode4662068,
          aarch64_aarch64_vector_transfer_vector_table_decode4663188,
          aarch64_unknown = -1
        }
        ;
        aarch64_isa_modes isa_mode;
        aarch64_opcodes opcode;
        uint32_t ir;
        bool decode(uint32_t isa_mode, uint64_t insn_pc, const uint8_t *ptr) override;
        JumpInfo get_jump_info() override;
        private:
        bool decode_aarch64(uint32_t ir);
      }
      ;
      class aarch64_decode_aarch64 : public aarch64_decode
      {
        public:
      }
      ;
      class aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4660422 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint32_t imm19;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4660422) <= 128, "Decode class for format fmt_branch_conditional_compare_decode4660422 is too big!");
      class aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4663551 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint32_t imm19;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4663551) <= 128, "Decode class for format fmt_branch_conditional_compare_decode4663551 is too big!");
      class aarch64_decode_aarch64_fmt_branch_conditional_cond_decode4661873 : public aarch64_decode_aarch64
      {
        public:
        uint32_t imm19;
        uint8_t cond;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_conditional_cond_decode4661873) <= 128, "Decode class for format fmt_branch_conditional_cond_decode4661873 is too big!");
      class aarch64_decode_aarch64_fmt_branch_conditional_test_decode4660675 : public aarch64_decode_aarch64
      {
        public:
        uint8_t b5;
        uint8_t b40;
        uint16_t imm14;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_conditional_test_decode4660675) <= 128, "Decode class for format fmt_branch_conditional_test_decode4660675 is too big!");
      class aarch64_decode_aarch64_fmt_branch_conditional_test_decode4661704 : public aarch64_decode_aarch64
      {
        public:
        uint8_t b5;
        uint8_t b40;
        uint16_t imm14;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_conditional_test_decode4661704) <= 128, "Decode class for format fmt_branch_conditional_test_decode4661704 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_dret_decode4660999 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_dret_decode4660999) <= 128, "Decode class for format fmt_branch_unconditional_dret_decode4660999 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_eret_decode4662816 : public aarch64_decode_aarch64
      {
        public:
        uint8_t M;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_eret_decode4662816) <= 128, "Decode class for format fmt_branch_unconditional_eret_decode4662816 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_eret_decode4662893 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_eret_decode4662893) <= 128, "Decode class for format fmt_branch_unconditional_eret_decode4662893 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_immediate_decode4660600 : public aarch64_decode_aarch64
      {
        public:
        uint32_t imm26;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_immediate_decode4660600) <= 128, "Decode class for format fmt_branch_unconditional_immediate_decode4660600 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_immediate_decode4662335 : public aarch64_decode_aarch64
      {
        public:
        uint32_t imm26;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_immediate_decode4662335) <= 128, "Decode class for format fmt_branch_unconditional_immediate_decode4662335 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4660339 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t M;
        uint8_t Rn;
        uint8_t Rm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4660339) <= 128, "Decode class for format fmt_branch_unconditional_register_decode4660339 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4661183 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4661183) <= 128, "Decode class for format fmt_branch_unconditional_register_decode4661183 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4661988 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4661988) <= 128, "Decode class for format fmt_branch_unconditional_register_decode4661988 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662511 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662511) <= 128, "Decode class for format fmt_branch_unconditional_register_decode4662511 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662554 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t M;
        uint8_t Rn;
        uint8_t Rm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662554) <= 128, "Decode class for format fmt_branch_unconditional_register_decode4662554 is too big!");
      class aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4663102 : public aarch64_decode_aarch64
      {
        public:
        uint8_t M;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4663102) <= 128, "Decode class for format fmt_branch_unconditional_register_decode4663102 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4659884 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4659884) <= 128, "Decode class for format fmt_float_arithmetic_addsub_decode4659884 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4660164 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4660164) <= 128, "Decode class for format fmt_float_arithmetic_addsub_decode4660164 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_div_decode4661050 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_div_decode4661050) <= 128, "Decode class for format fmt_float_arithmetic_div_decode4661050 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4660799 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4660799) <= 128, "Decode class for format fmt_float_arithmetic_maxmin_decode4660799 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4661464 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4661464) <= 128, "Decode class for format fmt_float_arithmetic_maxmin_decode4661464 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662587 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662587) <= 128, "Decode class for format fmt_float_arithmetic_maxmin_decode4662587 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662741 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662741) <= 128, "Decode class for format fmt_float_arithmetic_maxmin_decode4662741 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4659975 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4659975) <= 128, "Decode class for format fmt_float_arithmetic_mul_addsub_decode4659975 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4660987 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4660987) <= 128, "Decode class for format fmt_float_arithmetic_mul_addsub_decode4660987 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4661929 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4661929) <= 128, "Decode class for format fmt_float_arithmetic_mul_addsub_decode4661929 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4663122 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4663122) <= 128, "Decode class for format fmt_float_arithmetic_mul_addsub_decode4663122 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4661960 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4661960) <= 128, "Decode class for format fmt_float_arithmetic_mul_product_decode4661960 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4662459 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4662459) <= 128, "Decode class for format fmt_float_arithmetic_mul_product_decode4662459 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_32_64_decode4663342 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t op;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_32_64_decode4663342) <= 128, "Decode class for format fmt_float_arithmetic_round_frint_32_64_decode4663342 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661146 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661146) <= 128, "Decode class for format fmt_float_arithmetic_round_frint_decode4661146 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661406 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661406) <= 128, "Decode class for format fmt_float_arithmetic_round_frint_decode4661406 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661851 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661851) <= 128, "Decode class for format fmt_float_arithmetic_round_frint_decode4661851 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662480 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662480) <= 128, "Decode class for format fmt_float_arithmetic_round_frint_decode4662480 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662831 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662831) <= 128, "Decode class for format fmt_float_arithmetic_round_frint_decode4662831 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662913 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662913) <= 128, "Decode class for format fmt_float_arithmetic_round_frint_decode4662913 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4663447 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4663447) <= 128, "Decode class for format fmt_float_arithmetic_round_frint_decode4663447 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660079 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660079) <= 128, "Decode class for format fmt_float_arithmetic_unary_decode4660079 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660095 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660095) <= 128, "Decode class for format fmt_float_arithmetic_unary_decode4660095 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4661230 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4661230) <= 128, "Decode class for format fmt_float_arithmetic_unary_decode4661230 is too big!");
      class aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4662972 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4662972) <= 128, "Decode class for format fmt_float_arithmetic_unary_decode4662972 is too big!");
      class aarch64_decode_aarch64_fmt_float_compare_cond_decode4660204 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t nzcv;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_compare_cond_decode4660204) <= 128, "Decode class for format fmt_float_compare_cond_decode4660204 is too big!");
      class aarch64_decode_aarch64_fmt_float_compare_cond_decode4662650 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t nzcv;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_compare_cond_decode4662650) <= 128, "Decode class for format fmt_float_compare_cond_decode4662650 is too big!");
      class aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661696 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t opc;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661696) <= 128, "Decode class for format fmt_float_compare_uncond_decode4661696 is too big!");
      class aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661978 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t opc;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661978) <= 128, "Decode class for format fmt_float_compare_uncond_decode4661978 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_fix_decode4659868 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t scale;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_fix_decode4659868) <= 128, "Decode class for format fmt_float_convert_fix_decode4659868 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_fix_decode4660589 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t scale;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_fix_decode4660589) <= 128, "Decode class for format fmt_float_convert_fix_decode4660589 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_fix_decode4660609 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t scale;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_fix_decode4660609) <= 128, "Decode class for format fmt_float_convert_fix_decode4660609 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_fix_decode4663376 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t scale;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_fix_decode4663376) <= 128, "Decode class for format fmt_float_convert_fix_decode4663376 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_fp_decode4661993 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t opc;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_fp_decode4661993) <= 128, "Decode class for format fmt_float_convert_fp_decode4661993 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4660814 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4660814) <= 128, "Decode class for format fmt_float_convert_int_decode4660814 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4660937 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4660937) <= 128, "Decode class for format fmt_float_convert_int_decode4660937 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4661166 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4661166) <= 128, "Decode class for format fmt_float_convert_int_decode4661166 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4661592 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4661592) <= 128, "Decode class for format fmt_float_convert_int_decode4661592 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4661815 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4661815) <= 128, "Decode class for format fmt_float_convert_int_decode4661815 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4661967 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4661967) <= 128, "Decode class for format fmt_float_convert_int_decode4661967 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4662018 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t rmode;
        uint8_t opcode;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4662018) <= 128, "Decode class for format fmt_float_convert_int_decode4662018 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4662409 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4662409) <= 128, "Decode class for format fmt_float_convert_int_decode4662409 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4662431 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4662431) <= 128, "Decode class for format fmt_float_convert_int_decode4662431 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4662930 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4662930) <= 128, "Decode class for format fmt_float_convert_int_decode4662930 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4662989 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4662989) <= 128, "Decode class for format fmt_float_convert_int_decode4662989 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4663005 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4663005) <= 128, "Decode class for format fmt_float_convert_int_decode4663005 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4663285 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4663285) <= 128, "Decode class for format fmt_float_convert_int_decode4663285 is too big!");
      class aarch64_decode_aarch64_fmt_float_convert_int_decode4663836 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t typ;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_convert_int_decode4663836) <= 128, "Decode class for format fmt_float_convert_int_decode4663836 is too big!");
      class aarch64_decode_aarch64_fmt_float_move_fp_imm_decode4661440 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t imm8;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_move_fp_imm_decode4661440) <= 128, "Decode class for format fmt_float_move_fp_imm_decode4661440 is too big!");
      class aarch64_decode_aarch64_fmt_float_move_fp_select_decode4663824 : public aarch64_decode_aarch64
      {
        public:
        uint8_t typ;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_float_move_fp_select_decode4663824) <= 128, "Decode class for format fmt_float_move_fp_select_decode4663824 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4661401 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immlo;
        uint32_t immhi;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4661401) <= 128, "Decode class for format fmt_integer_arithmetic_address_pcrel_decode4661401 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4662813 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immlo;
        uint32_t immhi;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4662813) <= 128, "Decode class for format fmt_integer_arithmetic_address_pcrel_decode4662813 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4659891 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4659891) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_carry_decode4659891 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660241 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660241) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_carry_decode4660241 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660357 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660357) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_carry_decode4660357 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4661190 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4661190) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_carry_decode4661190 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t imm3;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_extendedreg_decode4659985 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t imm3;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_extendedreg_decode4660040 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t imm3;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_extendedreg_decode4661387 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t imm3;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_extendedreg_decode4663497 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4659928 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4659928) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_immediate_decode4659928 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660472 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660472) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_immediate_decode4660472 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660774 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660774) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_immediate_decode4660774 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4661399 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4661399) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_immediate_decode4661399 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_shiftedreg_decode4659762 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_shiftedreg_decode4660207 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_shiftedreg_decode4662735 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364) <= 128, "Decode class for format fmt_integer_arithmetic_addsub_shiftedreg_decode4663364 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4660660 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4660660) <= 128, "Decode class for format fmt_integer_arithmetic_cnt_decode4660660 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4663297 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4663297) <= 128, "Decode class for format fmt_integer_arithmetic_cnt_decode4663297 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4661638 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4661638) <= 128, "Decode class for format fmt_integer_arithmetic_div_decode4661638 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4663717 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4663717) <= 128, "Decode class for format fmt_integer_arithmetic_div_decode4663717 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4660051 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4660051) <= 128, "Decode class for format fmt_integer_arithmetic_mul_uniform_addsub_decode4660051 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4661006 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4661006) <= 128, "Decode class for format fmt_integer_arithmetic_mul_uniform_addsub_decode4661006 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4659759 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4659759) <= 128, "Decode class for format fmt_integer_arithmetic_mul_widening_3264_decode4659759 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660425 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660425) <= 128, "Decode class for format fmt_integer_arithmetic_mul_widening_3264_decode4660425 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660673 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660673) <= 128, "Decode class for format fmt_integer_arithmetic_mul_widening_3264_decode4660673 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4662921 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4662921) <= 128, "Decode class for format fmt_integer_arithmetic_mul_widening_3264_decode4662921 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4661763 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4661763) <= 128, "Decode class for format fmt_integer_arithmetic_mul_widening_64128hi_decode4661763 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4663750 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Ra;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4663750) <= 128, "Decode class for format fmt_integer_arithmetic_mul_widening_64128hi_decode4663750 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Xm;
        uint8_t Xn;
        uint8_t Xd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164) <= 128, "Decode class for format fmt_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Xm;
        uint8_t Xn;
        uint8_t Xd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191) <= 128, "Decode class for format fmt_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_rbit_decode4662535 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_rbit_decode4662535) <= 128, "Decode class for format fmt_integer_arithmetic_rbit_decode4662535 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4661605 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4661605) <= 128, "Decode class for format fmt_integer_arithmetic_rev_decode4661605 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663076 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t opc;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663076) <= 128, "Decode class for format fmt_integer_arithmetic_rev_decode4663076 is too big!");
      class aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663238 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663238) <= 128, "Decode class for format fmt_integer_arithmetic_rev_decode4663238 is too big!");
      class aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t N;
        uint8_t immr;
        uint8_t imms;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491) <= 128, "Decode class for format fmt_integer_bitfield_decode4661491 is too big!");
      class aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t N;
        uint8_t immr;
        uint8_t imms;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760) <= 128, "Decode class for format fmt_integer_bitfield_decode4662760 is too big!");
      class aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t N;
        uint8_t immr;
        uint8_t imms;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093) <= 128, "Decode class for format fmt_integer_bitfield_decode4663093 is too big!");
      class aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4660012 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t imm5;
        uint8_t cond;
        uint8_t Rn;
        uint8_t nzcv;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4660012) <= 128, "Decode class for format fmt_integer_conditional_compare_immediate_decode4660012 is too big!");
      class aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4662419 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t imm5;
        uint8_t cond;
        uint8_t Rn;
        uint8_t nzcv;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4662419) <= 128, "Decode class for format fmt_integer_conditional_compare_immediate_decode4662419 is too big!");
      class aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4660228 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t nzcv;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4660228) <= 128, "Decode class for format fmt_integer_conditional_compare_register_decode4660228 is too big!");
      class aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4662874 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t nzcv;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4662874) <= 128, "Decode class for format fmt_integer_conditional_compare_register_decode4662874 is too big!");
      class aarch64_decode_aarch64_fmt_integer_conditional_select_decode4660236 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_conditional_select_decode4660236) <= 128, "Decode class for format fmt_integer_conditional_select_decode4660236 is too big!");
      class aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661582 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661582) <= 128, "Decode class for format fmt_integer_conditional_select_decode4661582 is too big!");
      class aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661680 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661680) <= 128, "Decode class for format fmt_integer_conditional_select_decode4661680 is too big!");
      class aarch64_decode_aarch64_fmt_integer_conditional_select_decode4662744 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t cond;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_conditional_select_decode4662744) <= 128, "Decode class for format fmt_integer_conditional_select_decode4662744 is too big!");
      class aarch64_decode_aarch64_fmt_integer_crc_decode4660733 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_crc_decode4660733) <= 128, "Decode class for format fmt_integer_crc_decode4660733 is too big!");
      class aarch64_decode_aarch64_fmt_integer_crc_decode4662762 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_crc_decode4662762) <= 128, "Decode class for format fmt_integer_crc_decode4662762 is too big!");
      class aarch64_decode_aarch64_fmt_integer_flags_axflag_decode4662393 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_flags_axflag_decode4662393) <= 128, "Decode class for format fmt_integer_flags_axflag_decode4662393 is too big!");
      class aarch64_decode_aarch64_fmt_integer_flags_cfinv_decode4662708 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_flags_cfinv_decode4662708) <= 128, "Decode class for format fmt_integer_flags_cfinv_decode4662708 is too big!");
      class aarch64_decode_aarch64_fmt_integer_flags_rmif_decode4662797 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t mask;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_flags_rmif_decode4662797) <= 128, "Decode class for format fmt_integer_flags_rmif_decode4662797 is too big!");
      class aarch64_decode_aarch64_fmt_integer_flags_setf_decode4662979 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t sz;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_flags_setf_decode4662979) <= 128, "Decode class for format fmt_integer_flags_setf_decode4662979 is too big!");
      class aarch64_decode_aarch64_fmt_integer_flags_xaflag_decode4661273 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_flags_xaflag_decode4661273) <= 128, "Decode class for format fmt_integer_flags_xaflag_decode4661273 is too big!");
      class aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t N;
        uint8_t Rm;
        uint8_t imms;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192) <= 128, "Decode class for format fmt_integer_insext_extract_immediate_decode4661192 is too big!");
      class aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4660386 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t hw;
        uint16_t imm16;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4660386) <= 128, "Decode class for format fmt_integer_insext_insert_movewide_decode4660386 is too big!");
      class aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4661817 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t hw;
        uint16_t imm16;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4661817) <= 128, "Decode class for format fmt_integer_insext_insert_movewide_decode4661817 is too big!");
      class aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4662070 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t hw;
        uint16_t imm16;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4662070) <= 128, "Decode class for format fmt_integer_insext_insert_movewide_decode4662070 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t N;
        uint8_t immr;
        uint8_t imms;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553) <= 128, "Decode class for format fmt_integer_logical_immediate_decode4660553 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t N;
        uint8_t immr;
        uint8_t imms;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254) <= 128, "Decode class for format fmt_integer_logical_immediate_decode4662254 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t N;
        uint8_t immr;
        uint8_t imms;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411) <= 128, "Decode class for format fmt_integer_logical_immediate_decode4662411 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t N;
        uint8_t immr;
        uint8_t imms;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603) <= 128, "Decode class for format fmt_integer_logical_immediate_decode4662603 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725) <= 128, "Decode class for format fmt_integer_logical_shiftedreg_decode4660725 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895) <= 128, "Decode class for format fmt_integer_logical_shiftedreg_decode4660895 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912) <= 128, "Decode class for format fmt_integer_logical_shiftedreg_decode4660912 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645) <= 128, "Decode class for format fmt_integer_logical_shiftedreg_decode4662645 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175) <= 128, "Decode class for format fmt_integer_logical_shiftedreg_decode4663175 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421) <= 128, "Decode class for format fmt_integer_logical_shiftedreg_decode4663421 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481) <= 128, "Decode class for format fmt_integer_logical_shiftedreg_decode4663481 is too big!");
      class aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t shift;
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672) <= 128, "Decode class for format fmt_integer_logical_shiftedreg_decode4663672 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_autda_dp_1src_decode4660872 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_autda_dp_1src_decode4660872) <= 128, "Decode class for format fmt_integer_pac_autda_dp_1src_decode4660872 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_autdb_dp_1src_decode4660269 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_autdb_dp_1src_decode4660269) <= 128, "Decode class for format fmt_integer_pac_autdb_dp_1src_decode4660269 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_autia_dp_1src_decode4663841 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_autia_dp_1src_decode4663841) <= 128, "Decode class for format fmt_integer_pac_autia_dp_1src_decode4663841 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_autia_hint_decode4662773 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm_part1;
        uint8_t op2;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_autia_hint_decode4662773) <= 128, "Decode class for format fmt_integer_pac_autia_hint_decode4662773 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_autib_dp_1src_decode4659802 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_autib_dp_1src_decode4659802) <= 128, "Decode class for format fmt_integer_pac_autib_dp_1src_decode4659802 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_autib_hint_decode4663304 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm_part1;
        uint8_t op2;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_autib_hint_decode4663304) <= 128, "Decode class for format fmt_integer_pac_autib_hint_decode4663304 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_pacda_dp_1src_decode4663015 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_pacda_dp_1src_decode4663015) <= 128, "Decode class for format fmt_integer_pac_pacda_dp_1src_decode4663015 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_pacdb_dp_1src_decode4663160 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_pacdb_dp_1src_decode4663160) <= 128, "Decode class for format fmt_integer_pac_pacdb_dp_1src_decode4663160 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_pacga_dp_2src_decode4662751 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_pacga_dp_2src_decode4662751) <= 128, "Decode class for format fmt_integer_pac_pacga_dp_2src_decode4662751 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_pacia_dp_1src_decode4662350 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_pacia_dp_1src_decode4662350) <= 128, "Decode class for format fmt_integer_pac_pacia_dp_1src_decode4662350 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_pacia_hint_decode4661962 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm_part1;
        uint8_t op2;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_pacia_hint_decode4661962) <= 128, "Decode class for format fmt_integer_pac_pacia_hint_decode4661962 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_pacib_dp_1src_decode4662013 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Z;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_pacib_dp_1src_decode4662013) <= 128, "Decode class for format fmt_integer_pac_pacib_dp_1src_decode4662013 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_pacib_hint_decode4662575 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm_part1;
        uint8_t op2;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_pacib_hint_decode4662575) <= 128, "Decode class for format fmt_integer_pac_pacib_hint_decode4662575 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_strip_dp_1src_decode4663140 : public aarch64_decode_aarch64
      {
        public:
        uint8_t D;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_strip_dp_1src_decode4663140) <= 128, "Decode class for format fmt_integer_pac_strip_dp_1src_decode4663140 is too big!");
      class aarch64_decode_aarch64_fmt_integer_pac_strip_hint_decode4663565 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_pac_strip_hint_decode4663565) <= 128, "Decode class for format fmt_integer_pac_strip_hint_decode4663565 is too big!");
      class aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661093 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661093) <= 128, "Decode class for format fmt_integer_shift_variable_decode4661093 is too big!");
      class aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661771 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661771) <= 128, "Decode class for format fmt_integer_shift_variable_decode4661771 is too big!");
      class aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663366 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663366) <= 128, "Decode class for format fmt_integer_shift_variable_decode4663366 is too big!");
      class aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663826 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sf;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663826) <= 128, "Decode class for format fmt_integer_shift_variable_decode4663826 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcaddtag_decode4659982 : public aarch64_decode_aarch64
      {
        public:
        uint8_t uimm6;
        uint8_t op3;
        uint8_t uimm4;
        uint8_t Xn;
        uint8_t Xd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcaddtag_decode4659982) <= 128, "Decode class for format fmt_integer_tags_mcaddtag_decode4659982 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcgettag_decode4661013 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Xt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcgettag_decode4661013) <= 128, "Decode class for format fmt_integer_tags_mcgettag_decode4661013 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcgettagarray_decode4661787 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Xn;
        uint8_t Xt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcgettagarray_decode4661787) <= 128, "Decode class for format fmt_integer_tags_mcgettagarray_decode4661787 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcinsertrandomtag_decode4662727 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Xm;
        uint8_t Xn;
        uint8_t Xd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcinsertrandomtag_decode4662727) <= 128, "Decode class for format fmt_integer_tags_mcinsertrandomtag_decode4662727 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcinserttagmask_decode4659904 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Xm;
        uint8_t Xn;
        uint8_t Xd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcinserttagmask_decode4659904) <= 128, "Decode class for format fmt_integer_tags_mcinserttagmask_decode4659904 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettag_decode4662156 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettag_decode4662156) <= 128, "Decode class for format fmt_integer_tags_mcsettag_decode4662156 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapair_decode4659797 : public aarch64_decode_aarch64
      {
        public:
        uint8_t simm7;
        uint8_t Xt2;
        uint8_t Xn;
        uint8_t Xt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapair_decode4659797) <= 128, "Decode class for format fmt_integer_tags_mcsettaganddatapair_decode4659797 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpost_decode4663819 : public aarch64_decode_aarch64
      {
        public:
        uint8_t simm7;
        uint8_t Xt2;
        uint8_t Xn;
        uint8_t Xt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpost_decode4663819) <= 128, "Decode class for format fmt_integer_tags_mcsettaganddatapairpost_decode4663819 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpre_decode4660925 : public aarch64_decode_aarch64
      {
        public:
        uint8_t simm7;
        uint8_t Xt2;
        uint8_t Xn;
        uint8_t Xt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpre_decode4660925) <= 128, "Decode class for format fmt_integer_tags_mcsettaganddatapairpre_decode4660925 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodata_decode4662281 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodata_decode4662281) <= 128, "Decode class for format fmt_integer_tags_mcsettagandzerodata_decode4662281 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapost_decode4663163 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapost_decode4663163) <= 128, "Decode class for format fmt_integer_tags_mcsettagandzerodatapost_decode4663163 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapre_decode4660974 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapre_decode4660974) <= 128, "Decode class for format fmt_integer_tags_mcsettagandzerodatapre_decode4660974 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagarray_decode4662372 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Xn;
        uint8_t Xt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagarray_decode4662372) <= 128, "Decode class for format fmt_integer_tags_mcsettagarray_decode4662372 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagpair_decode4662758 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagpair_decode4662758) <= 128, "Decode class for format fmt_integer_tags_mcsettagpair_decode4662758 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodata_decode4661595 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodata_decode4661595) <= 128, "Decode class for format fmt_integer_tags_mcsettagpairandzerodata_decode4661595 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapost_decode4660126 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapost_decode4660126) <= 128, "Decode class for format fmt_integer_tags_mcsettagpairandzerodatapost_decode4660126 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapre_decode4663450 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapre_decode4663450) <= 128, "Decode class for format fmt_integer_tags_mcsettagpairandzerodatapre_decode4663450 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpost_decode4662003 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpost_decode4662003) <= 128, "Decode class for format fmt_integer_tags_mcsettagpairpost_decode4662003 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpre_decode4660116 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpre_decode4660116) <= 128, "Decode class for format fmt_integer_tags_mcsettagpairpre_decode4660116 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagpost_decode4663072 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagpost_decode4663072) <= 128, "Decode class for format fmt_integer_tags_mcsettagpost_decode4663072 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsettagpre_decode4663726 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Xn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsettagpre_decode4663726) <= 128, "Decode class for format fmt_integer_tags_mcsettagpre_decode4663726 is too big!");
      class aarch64_decode_aarch64_fmt_integer_tags_mcsubtag_decode4661222 : public aarch64_decode_aarch64
      {
        public:
        uint8_t uimm6;
        uint8_t op3;
        uint8_t uimm4;
        uint8_t Xn;
        uint8_t Xd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_integer_tags_mcsubtag_decode4661222) <= 128, "Decode class for format fmt_integer_tags_mcsubtag_decode4661222 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t L;
        uint8_t Rs;
        uint8_t o0;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064) <= 128, "Decode class for format fmt_memory_atomicops_cas_pair_decode4661064 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4660193 : public aarch64_decode_aarch64
      {
        public:
        uint8_t L;
        uint8_t Rs;
        uint8_t o0;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4660193) <= 128, "Decode class for format fmt_memory_atomicops_cas_single_decode4660193 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4662333 : public aarch64_decode_aarch64
      {
        public:
        uint8_t L;
        uint8_t Rs;
        uint8_t o0;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4662333) <= 128, "Decode class for format fmt_memory_atomicops_cas_single_decode4662333 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t L;
        uint8_t Rs;
        uint8_t o0;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354) <= 128, "Decode class for format fmt_memory_atomicops_cas_single_decode4663354 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4659787 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659842 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659842) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4659842 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659856 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659856) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4659856 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659889 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659889) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4659889 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659972 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659972) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4659972 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660233 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660233) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4660233 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660279 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660279) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4660279 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4660829 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661270 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661270) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4661270 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4661392 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661785 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661785) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4661785 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662136 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662136) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4662136 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4662278 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4662618 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4662655 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662804 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662804) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4662804 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662846 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662846) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4662846 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662879 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662879) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4662879 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663269 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663269) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4663269 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4663302 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663401 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663401) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4663401 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663632 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663632) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4663632 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663637 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663637) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4663637 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846) <= 128, "Decode class for format fmt_memory_atomicops_ld_decode4663846 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659773 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659773) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4659773 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659926 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659926) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4659926 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659967 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659967) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4659967 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660032 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660032) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4660032 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660362 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660362) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4660362 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660402 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660402) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4660402 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660527 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660527) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4660527 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660649 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660649) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4660649 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661098 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661098) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661098 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661130 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661130) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661130 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661151 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661151) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661151 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661241 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661241) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661241 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661326 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661326) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661326 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661555 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661555) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661555 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661654 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661654) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661654 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661690 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661690) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661690 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661709 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661709) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661709 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661726 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661726) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661726 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661840 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661840) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661840 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661856 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661856) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4661856 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4662436 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4662436) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4662436 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663086 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663086) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4663086 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663470 : public aarch64_decode_aarch64
      {
        public:
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663470) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4663470 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663768 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663768) <= 128, "Decode class for format fmt_memory_atomicops_st_decode4663768 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4660004 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4660004) <= 128, "Decode class for format fmt_memory_atomicops_swp_decode4660004 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192) <= 128, "Decode class for format fmt_memory_atomicops_swp_decode4662192 is too big!");
      class aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662871 : public aarch64_decode_aarch64
      {
        public:
        uint8_t A;
        uint8_t R;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662871) <= 128, "Decode class for format fmt_memory_atomicops_swp_decode4662871 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660282 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660282) <= 128, "Decode class for format fmt_memory_exclusive_pair_decode4660282 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660708 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660708) <= 128, "Decode class for format fmt_memory_exclusive_pair_decode4660708 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660763 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660763) <= 128, "Decode class for format fmt_memory_exclusive_pair_decode4660763 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4662692 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4662692) <= 128, "Decode class for format fmt_memory_exclusive_pair_decode4662692 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4659962 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4659962) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4659962 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660118 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660118) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4660118 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660782 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660782) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4660782 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660927 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660927) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4660927 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661421 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661421) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4661421 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661799 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661799) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4661799 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661995 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661995) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4661995 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4662818 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4662818) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4662818 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663044 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663044) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4663044 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663108 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663108) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4663108 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663386 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663386) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4663386 is too big!");
      class aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663615 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663615) <= 128, "Decode class for format fmt_memory_exclusive_single_decode4663615 is too big!");
      class aarch64_decode_aarch64_fmt_memory_literal_general_decode4660239 : public aarch64_decode_aarch64
      {
        public:
        uint32_t imm19;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_literal_general_decode4660239) <= 128, "Decode class for format fmt_memory_literal_general_decode4660239 is too big!");
      class aarch64_decode_aarch64_fmt_memory_literal_general_decode4660756 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint32_t imm19;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_literal_general_decode4660756) <= 128, "Decode class for format fmt_memory_literal_general_decode4660756 is too big!");
      class aarch64_decode_aarch64_fmt_memory_literal_general_decode4663539 : public aarch64_decode_aarch64
      {
        public:
        uint32_t imm19;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_literal_general_decode4663539) <= 128, "Decode class for format fmt_memory_literal_general_decode4663539 is too big!");
      class aarch64_decode_aarch64_fmt_memory_literal_simdfp_decode4661110 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint32_t imm19;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_literal_simdfp_decode4661110) <= 128, "Decode class for format fmt_memory_literal_simdfp_decode4661110 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4659830 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4659830) <= 128, "Decode class for format fmt_memory_ordered_decode4659830 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4659911 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4659911) <= 128, "Decode class for format fmt_memory_ordered_decode4659911 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4660053 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4660053) <= 128, "Decode class for format fmt_memory_ordered_decode4660053 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4660611 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4660611) <= 128, "Decode class for format fmt_memory_ordered_decode4660611 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4661081 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4661081) <= 128, "Decode class for format fmt_memory_ordered_decode4661081 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4662032 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4662032) <= 128, "Decode class for format fmt_memory_ordered_decode4662032 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4662426 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4662426) <= 128, "Decode class for format fmt_memory_ordered_decode4662426 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4663182 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4663182) <= 128, "Decode class for format fmt_memory_ordered_decode4663182 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4663316 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4663316) <= 128, "Decode class for format fmt_memory_ordered_decode4663316 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4663361 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4663361) <= 128, "Decode class for format fmt_memory_ordered_decode4663361 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4663419 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4663419) <= 128, "Decode class for format fmt_memory_ordered_decode4663419 is too big!");
      class aarch64_decode_aarch64_fmt_memory_ordered_decode4663670 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_ordered_decode4663670) <= 128, "Decode class for format fmt_memory_ordered_decode4663670 is too big!");
      class aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4660923 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4660923) <= 128, "Decode class for format fmt_memory_orderedrcpc_decode4660923 is too big!");
      class aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662110 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662110) <= 128, "Decode class for format fmt_memory_orderedrcpc_decode4662110 is too big!");
      class aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662823 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rs;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662823) <= 128, "Decode class for format fmt_memory_orderedrcpc_decode4662823 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc_part1;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572) <= 128, "Decode class for format fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc_part1;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826) <= 128, "Decode class for format fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4659909 : public aarch64_decode_aarch64
      {
        public:
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4659909) <= 128, "Decode class for format fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4659909 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc_part1;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225) <= 128, "Decode class for format fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc_part1;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715) <= 128, "Decode class for format fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc_part1;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481) <= 128, "Decode class for format fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc_part1;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204) <= 128, "Decode class for format fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784 : public aarch64_decode_aarch64
      {
        public:
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784) <= 128, "Decode class for format fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc_part1;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782) <= 128, "Decode class for format fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265 : public aarch64_decode_aarch64
      {
        public:
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265) <= 128, "Decode class for format fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc_part1;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621) <= 128, "Decode class for format fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915) <= 128, "Decode class for format fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706) <= 128, "Decode class for format fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951) <= 128, "Decode class for format fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580) <= 128, "Decode class for format fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613) <= 128, "Decode class for format fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267) <= 128, "Decode class for format fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639) <= 128, "Decode class for format fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639 is too big!");
      class aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t imm7;
        uint8_t Rt2;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559) <= 128, "Decode class for format fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489 : public aarch64_decode_aarch64
      {
        public:
        uint8_t M;
        uint8_t S;
        uint16_t imm9;
        uint8_t W;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_pac_decode4662489 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359) <= 128, "Decode class for format fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680) <= 128, "Decode class for format fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660430 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660430) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4660430 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4660435 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661209 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661209) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4661209 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661643 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661643) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4661643 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4661920 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4662851 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4662851) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4662851 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663396 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663396) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4663396 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4663570 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663642 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663642) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4663642 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747 : public aarch64_decode_aarch64
      {
        public:
        uint8_t opc;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747) <= 128, "Decode class for format fmt_memory_single_general_register_memory_single_general_register__decode4663747 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252) <= 128, "Decode class for format fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207) <= 128, "Decode class for format fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940) <= 128, "Decode class for format fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940) <= 128, "Decode class for format fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613) <= 128, "Decode class for format fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint16_t imm9;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145) <= 128, "Decode class for format fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315) <= 128, "Decode class for format fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint16_t imm12;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703) <= 128, "Decode class for format fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446) <= 128, "Decode class for format fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446 is too big!");
      class aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t opc_part1;
        uint8_t Rm;
        uint8_t option_name;
        uint8_t S;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887) <= 128, "Decode class for format fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551) <= 128, "Decode class for format fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022) <= 128, "Decode class for format fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425) <= 128, "Decode class for format fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630) <= 128, "Decode class for format fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386) <= 128, "Decode class for format fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part2;
        uint8_t opcode;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484) <= 128, "Decode class for format fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167) <= 128, "Decode class for format fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part2;
        uint8_t opcode;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650) <= 128, "Decode class for format fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999) <= 128, "Decode class for format fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343) <= 128, "Decode class for format fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part2;
        uint8_t opcode;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858) <= 128, "Decode class for format fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449) <= 128, "Decode class for format fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891) <= 128, "Decode class for format fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part2;
        uint8_t opcode;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493) <= 128, "Decode class for format fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106) <= 128, "Decode class for format fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474) <= 128, "Decode class for format fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646) <= 128, "Decode class for format fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506 is too big!");
      class aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t opcode_part1;
        uint8_t S;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776) <= 128, "Decode class for format fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776 is too big!");
      class aarch64_decode_aarch64_fmt_system_barriers_decode4659845 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_barriers_decode4659845) <= 128, "Decode class for format fmt_system_barriers_decode4659845 is too big!");
      class aarch64_decode_aarch64_fmt_system_barriers_decode4660260 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_barriers_decode4660260) <= 128, "Decode class for format fmt_system_barriers_decode4660260 is too big!");
      class aarch64_decode_aarch64_fmt_system_barriers_decode4660832 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_barriers_decode4660832) <= 128, "Decode class for format fmt_system_barriers_decode4660832 is too big!");
      class aarch64_decode_aarch64_fmt_system_barriers_decode4663379 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_barriers_decode4663379) <= 128, "Decode class for format fmt_system_barriers_decode4663379 is too big!");
      class aarch64_decode_aarch64_fmt_system_exceptions_debug_breakpoint_decode4662991 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm16;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_exceptions_debug_breakpoint_decode4662991) <= 128, "Decode class for format fmt_system_exceptions_debug_breakpoint_decode4662991 is too big!");
      class aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4661211 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm16;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4661211) <= 128, "Decode class for format fmt_system_exceptions_debug_exception_decode4661211 is too big!");
      class aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4662084 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm16;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4662084) <= 128, "Decode class for format fmt_system_exceptions_debug_exception_decode4662084 is too big!");
      class aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4663287 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm16;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4663287) <= 128, "Decode class for format fmt_system_exceptions_debug_exception_decode4663287 is too big!");
      class aarch64_decode_aarch64_fmt_system_exceptions_debug_halt_decode4662594 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm16;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_exceptions_debug_halt_decode4662594) <= 128, "Decode class for format fmt_system_exceptions_debug_halt_decode4662594 is too big!");
      class aarch64_decode_aarch64_fmt_system_exceptions_runtime_hvc_decode4662148 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm16;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_exceptions_runtime_hvc_decode4662148) <= 128, "Decode class for format fmt_system_exceptions_runtime_hvc_decode4662148 is too big!");
      class aarch64_decode_aarch64_fmt_system_exceptions_runtime_smc_decode4663617 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm16;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_exceptions_runtime_smc_decode4663617) <= 128, "Decode class for format fmt_system_exceptions_runtime_smc_decode4663617 is too big!");
      class aarch64_decode_aarch64_fmt_system_exceptions_runtime_svc_decode4662799 : public aarch64_decode_aarch64
      {
        public:
        uint16_t imm16;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_exceptions_runtime_svc_decode4662799) <= 128, "Decode class for format fmt_system_exceptions_runtime_svc_decode4662799 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4659837 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4659837) <= 128, "Decode class for format fmt_system_hints_decode4659837 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4659906 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4659906) <= 128, "Decode class for format fmt_system_hints_decode4659906 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4660103 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4660103) <= 128, "Decode class for format fmt_system_hints_decode4660103 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4660651 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4660651) <= 128, "Decode class for format fmt_system_hints_decode4660651 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4661636 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4661636) <= 128, "Decode class for format fmt_system_hints_decode4661636 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4662327 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm;
        uint8_t op2;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4662327) <= 128, "Decode class for format fmt_system_hints_decode4662327 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4662438 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4662438) <= 128, "Decode class for format fmt_system_hints_decode4662438 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4662596 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4662596) <= 128, "Decode class for format fmt_system_hints_decode4662596 is too big!");
      class aarch64_decode_aarch64_fmt_system_hints_decode4663225 : public aarch64_decode_aarch64
      {
        public:
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_hints_decode4663225) <= 128, "Decode class for format fmt_system_hints_decode4663225 is too big!");
      class aarch64_decode_aarch64_fmt_system_monitors_decode4660195 : public aarch64_decode_aarch64
      {
        public:
        uint8_t CRm;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_monitors_decode4660195) <= 128, "Decode class for format fmt_system_monitors_decode4660195 is too big!");
      class aarch64_decode_aarch64_fmt_system_register_cpsr_decode4660345 : public aarch64_decode_aarch64
      {
        public:
        uint8_t op1;
        uint8_t CRm;
        uint8_t op2;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_register_cpsr_decode4660345) <= 128, "Decode class for format fmt_system_register_cpsr_decode4660345 is too big!");
      class aarch64_decode_aarch64_fmt_system_register_system_decode4660215 : public aarch64_decode_aarch64
      {
        public:
        uint8_t o0;
        uint8_t op1;
        uint8_t CRn;
        uint8_t CRm;
        uint8_t op2;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_register_system_decode4660215) <= 128, "Decode class for format fmt_system_register_system_decode4660215 is too big!");
      class aarch64_decode_aarch64_fmt_system_register_system_decode4663228 : public aarch64_decode_aarch64
      {
        public:
        uint8_t o0;
        uint8_t op1;
        uint8_t CRn;
        uint8_t CRm;
        uint8_t op2;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_register_system_decode4663228) <= 128, "Decode class for format fmt_system_register_system_decode4663228 is too big!");
      class aarch64_decode_aarch64_fmt_system_sysops_decode4662440 : public aarch64_decode_aarch64
      {
        public:
        uint8_t op1;
        uint8_t CRn;
        uint8_t CRm;
        uint8_t op2;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_sysops_decode4662440) <= 128, "Decode class for format fmt_system_sysops_decode4662440 is too big!");
      class aarch64_decode_aarch64_fmt_system_sysops_decode4662507 : public aarch64_decode_aarch64
      {
        public:
        uint8_t op1;
        uint8_t CRn;
        uint8_t CRm;
        uint8_t op2;
        uint8_t Rt;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_system_sysops_decode4662507) <= 128, "Decode class for format fmt_system_sysops_decode4662507 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4659934 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4659934) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_diff_decode4659934 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4660959 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4660959) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_diff_decode4660959 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661040 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661040) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_diff_decode4661040 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661866 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661866) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_diff_decode4661866 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511) <= 128, "Decode class for format fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818 : public aarch64_decode_aarch64
      {
        public:
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877 : public aarch64_decode_aarch64
      {
        public:
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_int_decode4661468 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_long_decode4661161 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mul_long_decode4663607 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t rot;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662946 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662946) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662946 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661028 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661028) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661028 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392 : public aarch64_decode_aarch64
      {
        public:
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746 : public aarch64_decode_aarch64
      {
        public:
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4661367 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4661367) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_long_decode4661367 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662038 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662038) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_long_decode4662038 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662885 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662885) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_long_decode4662885 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t S;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t L;
        uint8_t M;
        uint8_t Rm;
        uint8_t S;
        uint8_t H;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187) <= 128, "Decode class for format fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4660854 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4660854) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_fp16_decode4660854 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4661141 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4661141) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_fp16_decode4661141 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t rot;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4661312 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4661312) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4661312 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_truncating_decode4659828 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_truncating_decode4659828) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_halving_truncating_decode4659828 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_truncating_decode4663677 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_truncating_decode4663677) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_halving_truncating_decode4663677 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_simd_decode4661901 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_simd_decode4661901) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_saturating_simd_decode4661901 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_simd_decode4661972 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_simd_decode4661972) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_saturating_simd_decode4661972 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660918 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660918) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_diff_decode4660918 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660953 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660953) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_diff_decode4660953 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4662320 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4662320) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_diff_decode4662320 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4663026 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4663026) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_diff_decode4663026 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_div_decode4661518 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_div_decode4661518) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_div_decode4661518 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_divfp16_decode4660971 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_divfp16_decode4660971) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_divfp16_decode4660971 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4659823 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4659823) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4659823 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4661345 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4661345) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4661345 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663314 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663314) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663314 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663460 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663460) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663460 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4661945 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4661945) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4661945 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4663049 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4663049) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4663049 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662273 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662273) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662273 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t rot;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t S;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t S;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode4662057 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode4662057) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode4662057 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode4663352 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode4663352) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_doubling_simd_decode4663352 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_product_decode4661569 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_product_decode4661569) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_product_decode4661569 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_product_decode4662314 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_product_decode4662314) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_mul_int_product_decode4662314 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_sisd_decode4661430 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_sisd_decode4661430) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_recps_sisd_decode4661430 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4660420 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4660420) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_simd_decode4660420 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4662287 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4662287) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_simd_decode4662287 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4662526 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4662526) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_simd_decode4662526 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663042 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663042) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663042 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663173 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663173) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663173 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663202 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663202) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663202 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663259 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663259) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663259 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663723 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663723) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_simd_decode4663723 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660309 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660309) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660309 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660780 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660780) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660780 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660793 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660793) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660793 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661201 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661201) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661201 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661257 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661257) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661257 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661912 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661912) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661912 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4662505 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4662505) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4662505 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4663213 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4663213) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4663213 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_int_decode4660804 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_int_decode4660804) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_int_decode4660804 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_int_decode4662778 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_int_decode4662778) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_int_decode4662778 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_simd_decode4661288 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_simd_decode4661288) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_saturating_simd_decode4661288 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_simd_decode4661331 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_simd_decode4661331) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_saturating_simd_decode4661331 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329) <= 128, "Decode class for format fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4660090 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4660090) <= 128, "Decode class for format fmt_vector_arithmetic_unary_add_pairwise_decode4660090 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661018 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661018) <= 128, "Decode class for format fmt_vector_arithmetic_unary_add_pairwise_decode4661018 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661475 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661475) <= 128, "Decode class for format fmt_vector_arithmetic_unary_add_pairwise_decode4661475 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4662243 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4662243) <= 128, "Decode class for format fmt_vector_arithmetic_unary_add_pairwise_decode4662243 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4660656 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4660656) <= 128, "Decode class for format fmt_vector_arithmetic_unary_add_saturating_simd_decode4660656 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4662464 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4662464) <= 128, "Decode class for format fmt_vector_arithmetic_unary_add_saturating_simd_decode4662464 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4660460 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4660460) <= 128, "Decode class for format fmt_vector_arithmetic_unary_add_saturating_sisd_decode4660460 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4662809 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4662809) <= 128, "Decode class for format fmt_vector_arithmetic_unary_add_saturating_sisd_decode4662809 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4662000 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4662000) <= 128, "Decode class for format fmt_vector_arithmetic_unary_clsz_decode4662000 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4663218 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4663218) <= 128, "Decode class for format fmt_vector_arithmetic_unary_clsz_decode4663218 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4660500 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4660500) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4660500 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662788 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662788) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662788 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662793 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662793) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662793 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662841 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662841) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662841 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode4663627 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode4663627) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode4663627 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cnt_decode4663731 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cnt_decode4663731) <= 128, "Decode class for format fmt_vector_arithmetic_unary_cnt_decode4663731 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4659939 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4659939) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_float_decode4659939 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4663755 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4663755) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_float_decode4663755 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4660470 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4660470) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_fp16_decode4660470 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4661115 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4661115) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_fp16_decode4661115 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4659980 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4659980) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_int_simd_decode4659980 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4660274 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4660274) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_int_simd_decode4660274 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4662856 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4662856) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4662856 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4663223 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4663223) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4663223 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4661871 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4661871) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4661871 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4662592 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4662592) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4662592 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626) <= 128, "Decode class for format fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_nosat_decode4660843 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_nosat_decode4660843) <= 128, "Decode class for format fmt_vector_arithmetic_unary_extract_nosat_decode4660843 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4660257 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4660257) <= 128, "Decode class for format fmt_vector_arithmetic_unary_extract_sat_simd_decode4660257 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4662161 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4662161) <= 128, "Decode class for format fmt_vector_arithmetic_unary_extract_sat_simd_decode4662161 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4660942 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4660942) <= 128, "Decode class for format fmt_vector_arithmetic_unary_extract_sat_sisd_decode4660942 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4663207 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4663207) <= 128, "Decode class for format fmt_vector_arithmetic_unary_extract_sat_sisd_decode4663207 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105) <= 128, "Decode class for format fmt_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131) <= 128, "Decode class for format fmt_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663150 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663150) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663150 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663155 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663155) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663155 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661340 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661340) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661340 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661822 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661822) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661822 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_narrow_decode4661792 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_narrow_decode4661792) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_narrow_decode4661792 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660314 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660314) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_round_decode4660314 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660577 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660577) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_round_decode4660577 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4661283 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4661283) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_round_decode4661283 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662228 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662228) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_round_decode4662228 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662675 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662675) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_round_decode4662675 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663556 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663556) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_round_decode4663556 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663586 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663586) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_round_decode4663586 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t U;
        uint8_t sz;
        uint8_t op;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_widen_decode4661107 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_widen_decode4661107) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_widen_decode4661107 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_simd_decode4662345 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_simd_decode4662345) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_xtn_simd_decode4662345 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_sisd_decode4660350 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_sisd_decode4660350) <= 128, "Decode class for format fmt_vector_arithmetic_unary_float_xtn_sisd_decode4660350 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4660074 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4660074) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4660074 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661361 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661361) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661361 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661397 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661397) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661397 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662175 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662175) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662175 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662531 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662531) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662531 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663264 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663264) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663264 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663309 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663309) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663309 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663807 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663807) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663807 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4660113 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4660113) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4660113 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4662689 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4662689) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4662689 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661445 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661445) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661445 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661667 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661667) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661667 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4659878 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4659878) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_round_decode4659878 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4660824 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4660824) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_round_decode4660824 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661251 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661251) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_round_decode4661251 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661757 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661757) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_round_decode4661757 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4662325 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4662325) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_round_decode4662325 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663091 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663091) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_round_decode4663091 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663248 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663248) <= 128, "Decode class for format fmt_vector_arithmetic_unary_fp16_round_decode4663248 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_not_decode4660992 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_not_decode4660992) <= 128, "Decode class for format fmt_vector_arithmetic_unary_not_decode4660992 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rbit_decode4662126 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rbit_decode4662126) <= 128, "Decode class for format fmt_vector_arithmetic_unary_rbit_decode4662126 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660516 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660516) <= 128, "Decode class for format fmt_vector_arithmetic_unary_rev_decode4660516 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660558 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660558) <= 128, "Decode class for format fmt_vector_arithmetic_unary_rev_decode4660558 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4663526 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4663526) <= 128, "Decode class for format fmt_vector_arithmetic_unary_rev_decode4663526 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_shift_decode4663581 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_shift_decode4663581) <= 128, "Decode class for format fmt_vector_arithmetic_unary_shift_decode4663581 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpx_decode4662670 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpx_decode4662670) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_frecpx_decode4662670 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpxfp16_decode4660511 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpxfp16_decode4660511) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_frecpxfp16_decode4660511 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_simd_decode4663020 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_simd_decode4663020) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_recip_float_simd_decode4663020 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_sisd_decode4660787 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_sisd_decode4660787) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_recip_float_sisd_decode4660787 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_sisd_decode4660212 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_sisd_decode4660212) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_recip_fp16_sisd_decode4660212 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_int_decode4661797 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_int_decode4661797) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_recip_int_decode4661797 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrt_decode4661827 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrt_decode4661827) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_sqrt_decode4661827 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode4660910 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode4660910) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode4660910 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_int_decode4660547 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_int_decode4660547) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_sqrtest_int_decode4660547 is too big!");
      class aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtfp16_decode4662238 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtfp16_decode4662238) <= 128, "Decode class for format fmt_vector_arithmetic_unary_special_sqrtfp16_decode4662238 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4660264 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4660264) <= 128, "Decode class for format fmt_vector_crypto_aes_mix_decode4660264 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4662866 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4662866) <= 128, "Decode class for format fmt_vector_crypto_aes_mix_decode4662866 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4660495 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4660495) <= 128, "Decode class for format fmt_vector_crypto_aes_round_decode4660495 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4661335 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4661335) <= 128, "Decode class for format fmt_vector_crypto_aes_round_decode4661335 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1hash_decode4660542 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1hash_decode4660542) <= 128, "Decode class for format fmt_vector_crypto_sha2op_sha1hash_decode4660542 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1sched1_decode4663437 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1sched1_decode4663437) <= 128, "Decode class for format fmt_vector_crypto_sha2op_sha1sched1_decode4663437 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha256sched0_decode4662364 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha256sched0_decode4662364) <= 128, "Decode class for format fmt_vector_crypto_sha2op_sha256sched0_decode4662364 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4661470 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4661470) <= 128, "Decode class for format fmt_vector_crypto_sha3_xar_decode4661470 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4662811 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t imm6;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4662811) <= 128, "Decode class for format fmt_vector_crypto_sha3_xar_decode4662811 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_choose_decode4662141 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_choose_decode4662141) <= 128, "Decode class for format fmt_vector_crypto_sha3op_sha1hash_choose_decode4662141 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_majority_decode4660158 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_majority_decode4660158) <= 128, "Decode class for format fmt_vector_crypto_sha3op_sha1hash_majority_decode4660158 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_parity_decode4662360 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_parity_decode4662360) <= 128, "Decode class for format fmt_vector_crypto_sha3op_sha1hash_parity_decode4662360 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1sched0_decode4661735 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1sched0_decode4661735) <= 128, "Decode class for format fmt_vector_crypto_sha3op_sha1sched0_decode4661735 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256hash_decode4660169 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256hash_decode4660169) <= 128, "Decode class for format fmt_vector_crypto_sha3op_sha256hash_decode4660169 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256hash_decode4660665 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256hash_decode4660665) <= 128, "Decode class for format fmt_vector_crypto_sha3op_sha256hash_decode4660665 is too big!");
      class aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256sched1_decode4660329 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256sched1_decode4660329) <= 128, "Decode class for format fmt_vector_crypto_sha3op_sha256sched1_decode4660329 is too big!");
      class aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t a;
        uint8_t b;
        uint8_t c;
        uint8_t d;
        uint8_t e;
        uint8_t f;
        uint8_t g;
        uint8_t h;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795) <= 128, "Decode class for format fmt_vector_fp16_movi_decode4659795 is too big!");
      class aarch64_decode_aarch64_fmt_vector_logical_decode4659791 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t op;
        uint8_t a;
        uint8_t b;
        uint8_t c;
        uint8_t cmode;
        uint8_t d;
        uint8_t e;
        uint8_t f;
        uint8_t g;
        uint8_t h;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_logical_decode4659791) <= 128, "Decode class for format fmt_vector_logical_decode4659791 is too big!");
      class aarch64_decode_aarch64_fmt_vector_logical_decode4660044 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t a;
        uint8_t b;
        uint8_t c;
        uint8_t cmode_part1;
        uint8_t d;
        uint8_t e;
        uint8_t f;
        uint8_t g;
        uint8_t h;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_logical_decode4660044) <= 128, "Decode class for format fmt_vector_logical_decode4660044 is too big!");
      class aarch64_decode_aarch64_fmt_vector_logical_decode4662755 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t op;
        uint8_t a;
        uint8_t b;
        uint8_t c;
        uint8_t d;
        uint8_t e;
        uint8_t f;
        uint8_t g;
        uint8_t h;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_logical_decode4662755) <= 128, "Decode class for format fmt_vector_logical_decode4662755 is too big!");
      class aarch64_decode_aarch64_fmt_vector_logical_decode4662766 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t a;
        uint8_t b;
        uint8_t c;
        uint8_t cmode;
        uint8_t d;
        uint8_t e;
        uint8_t f;
        uint8_t g;
        uint8_t h;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_logical_decode4662766) <= 128, "Decode class for format fmt_vector_logical_decode4662766 is too big!");
      class aarch64_decode_aarch64_fmt_vector_logical_decode4663664 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t a;
        uint8_t b;
        uint8_t c;
        uint8_t cmode_part1;
        uint8_t d;
        uint8_t e;
        uint8_t f;
        uint8_t g;
        uint8_t h;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_logical_decode4663664) <= 128, "Decode class for format fmt_vector_logical_decode4663664 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_add_simd_decode4662836 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_add_simd_decode4662836) <= 128, "Decode class for format fmt_vector_reduce_add_simd_decode4662836 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_add_sisd_decode4663031 : public aarch64_decode_aarch64
      {
        public:
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_add_sisd_decode4663031) <= 128, "Decode class for format fmt_vector_reduce_add_sisd_decode4663031 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4662233 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4662233) <= 128, "Decode class for format fmt_vector_reduce_addlong_decode4662233 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4663817 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4663817) <= 128, "Decode class for format fmt_vector_reduce_addlong_decode4663817 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16add_sisd_decode4660058 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16add_sisd_decode4660058) <= 128, "Decode class for format fmt_vector_reduce_fp16add_sisd_decode4660058 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4660037 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4660037) <= 128, "Decode class for format fmt_vector_reduce_fp16max_simd_decode4660037 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4661074 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4661074) <= 128, "Decode class for format fmt_vector_reduce_fp16max_simd_decode4661074 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4662961 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4662961) <= 128, "Decode class for format fmt_vector_reduce_fp16max_sisd_decode4662961 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4663455 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4663455) <= 128, "Decode class for format fmt_vector_reduce_fp16max_sisd_decode4663455 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4660108 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4660108) <= 128, "Decode class for format fmt_vector_reduce_fp16maxnm_simd_decode4660108 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4662749 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4662749) <= 128, "Decode class for format fmt_vector_reduce_fp16maxnm_simd_decode4662749 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4659916 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4659916) <= 128, "Decode class for format fmt_vector_reduce_fp16maxnm_sisd_decode4659916 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4663136 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4663136) <= 128, "Decode class for format fmt_vector_reduce_fp16maxnm_sisd_decode4663136 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpadd_sisd_decode4662713 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpadd_sisd_decode4662713) <= 128, "Decode class for format fmt_vector_reduce_fpadd_sisd_decode4662713 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4660185 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4660185) <= 128, "Decode class for format fmt_vector_reduce_fpmax_simd_decode4660185 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4662309 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4662309) <= 128, "Decode class for format fmt_vector_reduce_fpmax_simd_decode4662309 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661135 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661135) <= 128, "Decode class for format fmt_vector_reduce_fpmax_sisd_decode4661135 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661906 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661906) <= 128, "Decode class for format fmt_vector_reduce_fpmax_sisd_decode4661906 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4662935 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4662935) <= 128, "Decode class for format fmt_vector_reduce_fpmaxnm_simd_decode4662935 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4663384 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4663384) <= 128, "Decode class for format fmt_vector_reduce_fpmaxnm_simd_decode4663384 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4661626 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4661626) <= 128, "Decode class for format fmt_vector_reduce_fpmaxnm_sisd_decode4661626 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4663544 : public aarch64_decode_aarch64
      {
        public:
        uint8_t sz;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4663544) <= 128, "Decode class for format fmt_vector_reduce_fpmaxnm_sisd_decode4663544 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660141 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660141) <= 128, "Decode class for format fmt_vector_reduce_intmax_decode4660141 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660298 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660298) <= 128, "Decode class for format fmt_vector_reduce_intmax_decode4660298 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4661618 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4661618) <= 128, "Decode class for format fmt_vector_reduce_intmax_decode4661618 is too big!");
      class aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4662918 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4662918) <= 128, "Decode class for format fmt_vector_reduce_intmax_decode4662918 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4661860 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4661860) <= 128, "Decode class for format fmt_vector_shift_conv_float_simd_decode4661860 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4663763 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4663763) <= 128, "Decode class for format fmt_vector_shift_conv_float_simd_decode4663763 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4660737 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4660737) <= 128, "Decode class for format fmt_vector_shift_conv_float_sisd_decode4660737 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4662390 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4662390) <= 128, "Decode class for format fmt_vector_shift_conv_float_sisd_decode4662390 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4661170 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4661170) <= 128, "Decode class for format fmt_vector_shift_conv_int_simd_decode4661170 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4662247 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4662247) <= 128, "Decode class for format fmt_vector_shift_conv_int_simd_decode4662247 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4659995 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4659995) <= 128, "Decode class for format fmt_vector_shift_conv_int_sisd_decode4659995 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4660199 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4660199) <= 128, "Decode class for format fmt_vector_shift_conv_int_sisd_decode4660199 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_left_simd_decode4660772 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_left_simd_decode4660772) <= 128, "Decode class for format fmt_vector_shift_left_simd_decode4660772 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_left_sisd_decode4662659 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_left_sisd_decode4662659) <= 128, "Decode class for format fmt_vector_shift_left_sisd_decode4662659 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftinsert_simd_decode4662558 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftinsert_simd_decode4662558) <= 128, "Decode class for format fmt_vector_shift_leftinsert_simd_decode4662558 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftinsert_sisd_decode4661220 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftinsert_sisd_decode4661220) <= 128, "Decode class for format fmt_vector_shift_leftinsert_sisd_decode4661220 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4660621 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4660621) <= 128, "Decode class for format fmt_vector_shift_leftlong_decode4660621 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4661102 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4661102) <= 128, "Decode class for format fmt_vector_shift_leftlong_decode4661102 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4660748 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4660748) <= 128, "Decode class for format fmt_vector_shift_leftsat_simd_decode4660748 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661587 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661587) <= 128, "Decode class for format fmt_vector_shift_leftsat_simd_decode4661587 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661882 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661882) <= 128, "Decode class for format fmt_vector_shift_leftsat_simd_decode4661882 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4660585 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4660585) <= 128, "Decode class for format fmt_vector_shift_leftsat_sisd_decode4660585 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663536 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663536) <= 128, "Decode class for format fmt_vector_shift_leftsat_sisd_decode4663536 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663736 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663736) <= 128, "Decode class for format fmt_vector_shift_leftsat_sisd_decode4663736 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660863 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660863) <= 128, "Decode class for format fmt_vector_shift_right_simd_decode4660863 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660888 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660888) <= 128, "Decode class for format fmt_vector_shift_right_simd_decode4660888 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661496 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661496) <= 128, "Decode class for format fmt_vector_shift_right_simd_decode4661496 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661574 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661574) <= 128, "Decode class for format fmt_vector_shift_right_simd_decode4661574 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662100 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662100) <= 128, "Decode class for format fmt_vector_shift_right_simd_decode4662100 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662298 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662298) <= 128, "Decode class for format fmt_vector_shift_right_simd_decode4662298 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663494 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663494) <= 128, "Decode class for format fmt_vector_shift_right_simd_decode4663494 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663531 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663531) <= 128, "Decode class for format fmt_vector_shift_right_simd_decode4663531 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660009 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660009) <= 128, "Decode class for format fmt_vector_shift_right_sisd_decode4660009 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660893 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660893) <= 128, "Decode class for format fmt_vector_shift_right_sisd_decode4660893 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660984 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660984) <= 128, "Decode class for format fmt_vector_shift_right_sisd_decode4660984 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4661579 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4661579) <= 128, "Decode class for format fmt_vector_shift_right_sisd_decode4661579 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662095 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662095) <= 128, "Decode class for format fmt_vector_shift_right_sisd_decode4662095 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662908 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662908) <= 128, "Decode class for format fmt_vector_shift_right_sisd_decode4662908 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663597 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663597) <= 128, "Decode class for format fmt_vector_shift_right_sisd_decode4663597 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663695 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663695) <= 128, "Decode class for format fmt_vector_shift_right_sisd_decode4663695 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightinsert_simd_decode4660867 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightinsert_simd_decode4660867) <= 128, "Decode class for format fmt_vector_shift_rightinsert_simd_decode4660867 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightinsert_sisd_decode4662995 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightinsert_sisd_decode4662995) <= 128, "Decode class for format fmt_vector_shift_rightinsert_sisd_decode4662995 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4661246 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4661246) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_logical_decode4661246 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4663010 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4663010) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_logical_decode4663010 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662115 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662115) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662115 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660700 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660700) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660700 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660979 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660979) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660979 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4660319 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4660319) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_uniform_simd_decode4660319 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662217 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662217) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_uniform_simd_decode4662217 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662628 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662628) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_uniform_simd_decode4662628 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4663802 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4663802) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_uniform_simd_decode4663802 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661069 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661069) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_uniform_sisd_decode4661069 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661527 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661527) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_uniform_sisd_decode4661527 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662450 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662450) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_uniform_sisd_decode4662450 is too big!");
      class aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662573 : public aarch64_decode_aarch64
      {
        public:
        uint8_t immh;
        uint8_t immb;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662573) <= 128, "Decode class for format fmt_vector_shift_rightnarrow_uniform_sisd_decode4662573 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_integer_dup_decode4659921 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t imm5;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_integer_dup_decode4659921) <= 128, "Decode class for format fmt_vector_transfer_integer_dup_decode4659921 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_integer_insert_decode4661896 : public aarch64_decode_aarch64
      {
        public:
        uint8_t imm5;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_integer_insert_decode4661896) <= 128, "Decode class for format fmt_vector_transfer_integer_insert_decode4661896 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_integer_move_signed_decode4662984 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t imm5;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_integer_move_signed_decode4662984) <= 128, "Decode class for format fmt_vector_transfer_integer_move_signed_decode4662984 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_integer_move_unsigned_decode4660293 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t imm5;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_integer_move_unsigned_decode4660293) <= 128, "Decode class for format fmt_vector_transfer_integer_move_unsigned_decode4660293 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_simd_decode4660598 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t imm5;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_simd_decode4660598) <= 128, "Decode class for format fmt_vector_transfer_vector_cpydup_simd_decode4660598 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_sisd_decode4663417 : public aarch64_decode_aarch64
      {
        public:
        uint8_t imm5;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_sisd_decode4663417) <= 128, "Decode class for format fmt_vector_transfer_vector_cpydup_sisd_decode4663417 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_extract_decode4663603 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t imm4;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_extract_decode4663603) <= 128, "Decode class for format fmt_vector_transfer_vector_extract_decode4663603 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_insert_decode4663098 : public aarch64_decode_aarch64
      {
        public:
        uint8_t imm5;
        uint8_t imm4;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_insert_decode4663098) <= 128, "Decode class for format fmt_vector_transfer_vector_insert_decode4663098 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4660681 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4660681) <= 128, "Decode class for format fmt_vector_transfer_vector_permute_transpose_decode4660681 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4661702 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4661702) <= 128, "Decode class for format fmt_vector_transfer_vector_permute_transpose_decode4661702 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4660564 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4660564) <= 128, "Decode class for format fmt_vector_transfer_vector_permute_unzip_decode4660564 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4661752 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4661752) <= 128, "Decode class for format fmt_vector_transfer_vector_permute_unzip_decode4661752 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4660731 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4660731) <= 128, "Decode class for format fmt_vector_transfer_vector_permute_zip_decode4660731 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4661321 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t size;
        uint8_t Rm;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4661321) <= 128, "Decode class for format fmt_vector_transfer_vector_permute_zip_decode4661321 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4662068 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t len;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4662068) <= 128, "Decode class for format fmt_vector_transfer_vector_table_decode4662068 is too big!");
      class aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4663188 : public aarch64_decode_aarch64
      {
        public:
        uint8_t Q;
        uint8_t Rm;
        uint8_t len;
        uint8_t Rn;
        uint8_t Rd;
        inline void decode_behaviour()
        {
        }
      }
      ;
      static_assert(sizeof(aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4663188) <= 128, "Decode class for format fmt_vector_transfer_vector_table_decode4663188 is too big!");
    }
  }
}
