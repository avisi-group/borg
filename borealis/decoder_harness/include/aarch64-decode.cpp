#include <aarch64-decode.h>
using namespace captive::arch::aarch64;
bool aarch64_decode::decode(uint32_t isa_mode, uint64_t insn_pc, const uint8_t *ptr)
{
  opcode = aarch64_unknown;
  pc = insn_pc;
  ir = *(const uint32_t *)ptr;
  end_of_block = false;
  is_predicated = false;
  bool result = false;
  switch ((aarch64_isa_modes)isa_mode) 
  {
  case aarch64_aarch64:
    result = decode_aarch64(ir);
    break;
  }
  if (opcode == aarch64_unknown) 
  {
    end_of_block = true;
    result = true;
  }
  return result;
}
captive::arch::JumpInfo aarch64_decode::get_jump_info()
{
  JumpInfo info;
  info.type = captive::arch::JumpInfo::NONE;
  info.target = 0;
  switch (opcode) 
  {
  case aarch64_unknown:
    info.type = captive::arch::JumpInfo::INDIRECT;
    break;
  default: break;
  }
  return info;
}
inline bool aarch64_decode::decode_aarch64(uint32_t ir)
{
  isa_mode = aarch64_aarch64;
  // Node 0
  switch (UNSIGNED_BITS(ir, 31,0)) 
  {
  case 3613328352:
    {
      // Node 14
      opcode = aarch64_aarch64_branch_unconditional_dret_decode4660999;
      length = 4;
      ((aarch64_decode_aarch64_fmt_branch_unconditional_dret_decode4660999&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 763298784:
    {
      // Node 18
      opcode = aarch64_aarch64_branch_unconditional_eret_decode4662893;
      length = 4;
      ((aarch64_decode_aarch64_fmt_branch_unconditional_eret_decode4662893&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 537571576:
    {
      // Node 318
      opcode = aarch64_aarch64_integer_flags_cfinv_decode4662708;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_flags_cfinv_decode4662708&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,10)) 
  {
  case 257728:
    {
      // Node 156
      // Node 157
      opcode = aarch64_aarch64_float_convert_int_decode4663285;
      length = 4;
      ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663285&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663285&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663285&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 3364865:
    {
      // Node 267
      // Node 268
      opcode = aarch64_aarch64_integer_arithmetic_rev_decode4661605;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4661605&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4661605&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4661605&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 2375851:
    {
      // Node 428
      // Node 429
      opcode = aarch64_aarch64_integer_tags_mcgettagarray_decode4661787;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_tags_mcgettagarray_decode4661787&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcgettagarray_decode4661787&)*this).Xt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcgettagarray_decode4661787&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 278699:
    {
      // Node 452
      // Node 453
      opcode = aarch64_aarch64_integer_tags_mcsettagarray_decode4662372;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagarray_decode4662372&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagarray_decode4662372&)*this).Xt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagarray_decode4662372&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 1999489:
    {
      // Node 2103
      // Node 2104
      opcode = aarch64_aarch64_vector_crypto_sha2op_sha1hash_decode4660542;
      length = 4;
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1hash_decode4660542&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1hash_decode4660542&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1hash_decode4660542&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 1999553:
    {
      // Node 2105
      // Node 2106
      opcode = aarch64_aarch64_vector_crypto_sha2op_sha1sched1_decode4663437;
      length = 4;
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1sched1_decode4663437&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1sched1_decode4663437&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha1sched1_decode4663437&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 1999521:
    {
      // Node 2107
      // Node 2108
      opcode = aarch64_aarch64_vector_crypto_sha2op_sha256sched0_decode4662364;
      length = 4;
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha256sched0_decode4662364&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha256sched0_decode4662364&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha2op_sha256sched0_decode4662364&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 2220505:
    {
      // Node 2154
      // Node 2155
      opcode = aarch64_aarch64_vector_reduce_fp16add_sisd_decode4660058;
      length = 4;
      ((aarch64_decode_aarch64_fmt_vector_reduce_fp16add_sisd_decode4660058&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_vector_reduce_fp16add_sisd_decode4660058&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_vector_reduce_fp16add_sisd_decode4660058&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,11)) 
  {
  case 372705:
    {
      // Node 15
      // Node 16
      switch (UNSIGNED_BITS(ir, 9,0)) 
      {
      case 1023:
        {
          // Node 17
          opcode = aarch64_aarch64_branch_unconditional_eret_decode4662816;
          length = 4;
          ((aarch64_decode_aarch64_fmt_branch_unconditional_eret_decode4662816&)*this).M = BITSEL(ir, 10);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_eret_decode4662816&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1682946:
    {
      // Node 409
      // Node 410
      switch (UNSIGNED_BITS(ir, 9,5)) 
      {
      case 31:
        {
          // Node 411
          // Node 412
          opcode = aarch64_aarch64_integer_pac_strip_dp_1src_decode4663140;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_strip_dp_1src_decode4663140&)*this).D = BITSEL(ir, 10);
          ((aarch64_decode_aarch64_fmt_integer_pac_strip_dp_1src_decode4663140&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_strip_dp_1src_decode4663140&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,12)) 
  {
  case 131243:
    {
      // Node 315
      // Node 316
      switch (UNSIGNED_BITS(ir, 7,0)) 
      {
      case 250:
        {
          // Node 317
          opcode = aarch64_aarch64_integer_flags_axflag_decode4662393;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_flags_axflag_decode4662393&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
          ((aarch64_decode_aarch64_fmt_integer_flags_axflag_decode4662393&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 252:
        {
          // Node 330
          opcode = aarch64_aarch64_integer_flags_xaflag_decode4661273;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_flags_xaflag_decode4661273&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
          ((aarch64_decode_aarch64_fmt_integer_flags_xaflag_decode4661273&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,14)) 
  {
  case 210368:
    {
      // Node 373
      // Node 374
      switch (UNSIGNED_BITS(ir, 12,10)) 
      {
      case 3:
        {
          // Node 375
          // Node 376
          opcode = aarch64_aarch64_integer_pac_autda_dp_1src_decode4660872;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_autda_dp_1src_decode4660872&)*this).Z = BITSEL(ir, 13);
          ((aarch64_decode_aarch64_fmt_integer_pac_autda_dp_1src_decode4660872&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_autda_dp_1src_decode4660872&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_autda_dp_1src_decode4660872&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 7:
        {
          // Node 377
          // Node 378
          opcode = aarch64_aarch64_integer_pac_autdb_dp_1src_decode4660269;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_autdb_dp_1src_decode4660269&)*this).Z = BITSEL(ir, 13);
          ((aarch64_decode_aarch64_fmt_integer_pac_autdb_dp_1src_decode4660269&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_autdb_dp_1src_decode4660269&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_autdb_dp_1src_decode4660269&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 379
          // Node 380
          opcode = aarch64_aarch64_integer_pac_autia_dp_1src_decode4663841;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_autia_dp_1src_decode4663841&)*this).Z = BITSEL(ir, 13);
          ((aarch64_decode_aarch64_fmt_integer_pac_autia_dp_1src_decode4663841&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_autia_dp_1src_decode4663841&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_autia_dp_1src_decode4663841&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 5:
        {
          // Node 386
          // Node 387
          opcode = aarch64_aarch64_integer_pac_autib_dp_1src_decode4659802;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_autib_dp_1src_decode4659802&)*this).Z = BITSEL(ir, 13);
          ((aarch64_decode_aarch64_fmt_integer_pac_autib_dp_1src_decode4659802&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_autib_dp_1src_decode4659802&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_autib_dp_1src_decode4659802&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 391
          // Node 392
          opcode = aarch64_aarch64_integer_pac_pacda_dp_1src_decode4663015;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_pacda_dp_1src_decode4663015&)*this).Z = BITSEL(ir, 13);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacda_dp_1src_decode4663015&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacda_dp_1src_decode4663015&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacda_dp_1src_decode4663015&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 6:
        {
          // Node 393
          // Node 394
          opcode = aarch64_aarch64_integer_pac_pacdb_dp_1src_decode4663160;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_pacdb_dp_1src_decode4663160&)*this).Z = BITSEL(ir, 13);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacdb_dp_1src_decode4663160&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacdb_dp_1src_decode4663160&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacdb_dp_1src_decode4663160&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 399
          // Node 400
          opcode = aarch64_aarch64_integer_pac_pacia_dp_1src_decode4662350;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_pacia_dp_1src_decode4662350&)*this).Z = BITSEL(ir, 13);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacia_dp_1src_decode4662350&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacia_dp_1src_decode4662350&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacia_dp_1src_decode4662350&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 4:
        {
          // Node 404
          // Node 405
          opcode = aarch64_aarch64_integer_pac_pacib_dp_1src_decode4662013;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_pacib_dp_1src_decode4662013&)*this).Z = BITSEL(ir, 13);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacib_dp_1src_decode4662013&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacib_dp_1src_decode4662013&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacib_dp_1src_decode4662013&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,19)) 
  {
  case 1374:
    {
      // Node 1191
      // Node 1192
      opcode = aarch64_aarch64_system_sysops_decode4662440;
      length = 4;
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662440&)*this).op1 = UNSIGNED_BITS(ir, 18,16);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662440&)*this).CRn = UNSIGNED_BITS(ir, 15,12);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662440&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662440&)*this).op2 = UNSIGNED_BITS(ir, 7,5);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662440&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662440&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 1370:
    {
      // Node 1193
      // Node 1194
      opcode = aarch64_aarch64_system_sysops_decode4662507;
      length = 4;
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662507&)*this).op1 = UNSIGNED_BITS(ir, 18,16);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662507&)*this).CRn = UNSIGNED_BITS(ir, 15,12);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662507&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662507&)*this).op2 = UNSIGNED_BITS(ir, 7,5);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662507&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_system_sysops_decode4662507&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,20)) 
  {
  case 685:
    {
      // Node 1187
      // Node 1188
      opcode = aarch64_aarch64_system_register_system_decode4660215;
      length = 4;
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4660215&)*this).o0 = BITSEL(ir, 19);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4660215&)*this).op1 = UNSIGNED_BITS(ir, 18,16);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4660215&)*this).CRn = UNSIGNED_BITS(ir, 15,12);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4660215&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4660215&)*this).op2 = UNSIGNED_BITS(ir, 7,5);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4660215&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4660215&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 687:
    {
      // Node 1189
      // Node 1190
      opcode = aarch64_aarch64_system_register_system_decode4663228;
      length = 4;
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4663228&)*this).o0 = BITSEL(ir, 19);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4663228&)*this).op1 = UNSIGNED_BITS(ir, 18,16);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4663228&)*this).CRn = UNSIGNED_BITS(ir, 15,12);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4663228&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4663228&)*this).op2 = UNSIGNED_BITS(ir, 7,5);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4663228&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_system_register_system_decode4663228&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 684:
    {
      // Node 2357
      switch (UNSIGNED_BITS(ir, 19,11)) 
      {
      case 200:
        {
          // Node 2366
          switch (UNSIGNED_BITS(ir, 10,0)) 
          {
          case 255:
            {
              // Node 413
              opcode = aarch64_aarch64_integer_pac_strip_hint_decode4663565;
              length = 4;
              ((aarch64_decode_aarch64_fmt_integer_pac_strip_hint_decode4663565&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1055:
            {
              // Node 1170
              opcode = aarch64_aarch64_system_hints_decode4659837;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4659837&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 95:
            {
              // Node 1171
              opcode = aarch64_aarch64_system_hints_decode4659906;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4659906&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 31:
            {
              // Node 1172
              opcode = aarch64_aarch64_system_hints_decode4660103;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4660103&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 191:
            {
              // Node 1173
              opcode = aarch64_aarch64_system_hints_decode4660651;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4660651&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 63:
            {
              // Node 1174
              opcode = aarch64_aarch64_system_hints_decode4661636;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4661636&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1183:
            {
              // Node 1178
              opcode = aarch64_aarch64_system_hints_decode4662438;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4662438&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 159:
            {
              // Node 1179
              opcode = aarch64_aarch64_system_hints_decode4662596;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4662596&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 223:
            {
              // Node 1180
              opcode = aarch64_aarch64_system_hints_decode4663225;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4663225&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 381
              // Node 382
              switch (UNSIGNED_BITS(ir, 8,6)) 
              {
              case 5:
                {
                  // Node 383
                  // Node 384
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 385
                      opcode = aarch64_aarch64_integer_pac_autia_hint_decode4662773;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_integer_pac_autia_hint_decode4662773&)*this).CRm_part1 = BITSEL(ir, 9);
                      ((aarch64_decode_aarch64_fmt_integer_pac_autia_hint_decode4662773&)*this).op2 = BITSEL(ir, 5);
                      ((aarch64_decode_aarch64_fmt_integer_pac_autia_hint_decode4662773&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 7:
                {
                  // Node 388
                  // Node 389
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 390
                      opcode = aarch64_aarch64_integer_pac_autib_hint_decode4663304;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_integer_pac_autib_hint_decode4663304&)*this).CRm_part1 = BITSEL(ir, 9);
                      ((aarch64_decode_aarch64_fmt_integer_pac_autib_hint_decode4663304&)*this).op2 = BITSEL(ir, 5);
                      ((aarch64_decode_aarch64_fmt_integer_pac_autib_hint_decode4663304&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 4:
                {
                  // Node 401
                  // Node 402
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 403
                      opcode = aarch64_aarch64_integer_pac_pacia_hint_decode4661962;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_integer_pac_pacia_hint_decode4661962&)*this).CRm_part1 = BITSEL(ir, 9);
                      ((aarch64_decode_aarch64_fmt_integer_pac_pacia_hint_decode4661962&)*this).op2 = BITSEL(ir, 5);
                      ((aarch64_decode_aarch64_fmt_integer_pac_pacia_hint_decode4661962&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 6:
                {
                  // Node 406
                  // Node 407
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 408
                      opcode = aarch64_aarch64_integer_pac_pacib_hint_decode4662575;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_integer_pac_pacib_hint_decode4662575&)*this).CRm_part1 = BITSEL(ir, 9);
                      ((aarch64_decode_aarch64_fmt_integer_pac_pacib_hint_decode4662575&)*this).op2 = BITSEL(ir, 5);
                      ((aarch64_decode_aarch64_fmt_integer_pac_pacib_hint_decode4662575&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 19,12)) 
      {
      case 108:
        {
          // Node 1148
          // Node 1149
          switch (UNSIGNED_BITS(ir, 7,0)) 
          {
          case 191:
            {
              // Node 1150
              opcode = aarch64_aarch64_system_barriers_decode4659845;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_barriers_decode4659845&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
              ((aarch64_decode_aarch64_fmt_system_barriers_decode4659845&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 223:
            {
              // Node 1151
              opcode = aarch64_aarch64_system_barriers_decode4660260;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_barriers_decode4660260&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
              ((aarch64_decode_aarch64_fmt_system_barriers_decode4660260&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 159:
            {
              // Node 1152
              opcode = aarch64_aarch64_system_barriers_decode4660832;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_barriers_decode4660832&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
              ((aarch64_decode_aarch64_fmt_system_barriers_decode4660832&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 255:
            {
              // Node 1153
              opcode = aarch64_aarch64_system_barriers_decode4663379;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_barriers_decode4663379&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
              ((aarch64_decode_aarch64_fmt_system_barriers_decode4663379&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 95:
            {
              // Node 1181
              opcode = aarch64_aarch64_system_monitors_decode4660195;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_monitors_decode4660195&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
              ((aarch64_decode_aarch64_fmt_system_monitors_decode4660195&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 100:
        {
          // Node 1175
          // Node 1176
          switch (UNSIGNED_BITS(ir, 4,0)) 
          {
          case 31:
            {
              // Node 1177
              opcode = aarch64_aarch64_system_hints_decode4662327;
              length = 4;
              ((aarch64_decode_aarch64_fmt_system_hints_decode4662327&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
              ((aarch64_decode_aarch64_fmt_system_hints_decode4662327&)*this).op2 = UNSIGNED_BITS(ir, 7,5);
              ((aarch64_decode_aarch64_fmt_system_hints_decode4662327&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch ((ir & BIT_LSB(19)) >> 19)
      {
      case 0:
        {
          // Node 1182
          // Node 1183
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 2:
            {
              // Node 1184
              // Node 1185
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 1186
                  opcode = aarch64_aarch64_system_register_cpsr_decode4660345;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_system_register_cpsr_decode4660345&)*this).op1 = UNSIGNED_BITS(ir, 18,16);
                  ((aarch64_decode_aarch64_fmt_system_register_cpsr_decode4660345&)*this).CRm = UNSIGNED_BITS(ir, 11,8);
                  ((aarch64_decode_aarch64_fmt_system_register_cpsr_decode4660345&)*this).op2 = UNSIGNED_BITS(ir, 7,5);
                  ((aarch64_decode_aarch64_fmt_system_register_cpsr_decode4660345&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,21)) 
  {
  case 1242:
    {
      // Node 237
      // Node 238
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 1:
        {
          // Node 239
          // Node 240
          opcode = aarch64_aarch64_integer_arithmetic_mul_widening_3264_decode4659759;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4659759&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4659759&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4659759&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4659759&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4659759&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 245
          // Node 246
          opcode = aarch64_aarch64_integer_arithmetic_mul_widening_3264_decode4660673;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660673&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660673&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660673&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660673&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660673&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1246:
    {
      // Node 241
      // Node 242
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 243
          // Node 244
          opcode = aarch64_aarch64_integer_arithmetic_mul_widening_3264_decode4660425;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660425&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660425&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660425&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660425&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4660425&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 247
          // Node 248
          opcode = aarch64_aarch64_integer_arithmetic_mul_widening_3264_decode4662921;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4662921&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4662921&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4662921&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4662921&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_3264_decode4662921&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1245:
    {
      // Node 249
      // Node 250
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 251
          // Node 252
          opcode = aarch64_aarch64_integer_arithmetic_mul_widening_64128hi_decode4661763;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4661763&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4661763&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4661763&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4661763&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4661763&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1241:
    {
      // Node 253
      // Node 254
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 255
          // Node 256
          opcode = aarch64_aarch64_integer_arithmetic_mul_widening_64128hi_decode4663750;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4663750&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4663750&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4663750&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4663750&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_widening_64128hi_decode4663750&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 857:
    {
      // Node 257
      // Node 258
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 0:
        {
          // Node 259
          // Node 260
          opcode = aarch64_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164&)*this).Xm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164&)*this).Xd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddress_decode4662164&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 8:
        {
          // Node 430
          // Node 431
          opcode = aarch64_aarch64_integer_tags_mcinsertrandomtag_decode4662727;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcinsertrandomtag_decode4662727&)*this).Xm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcinsertrandomtag_decode4662727&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcinsertrandomtag_decode4662727&)*this).Xd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcinsertrandomtag_decode4662727&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 40:
        {
          // Node 432
          // Node 433
          opcode = aarch64_aarch64_integer_tags_mcinserttagmask_decode4659904;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcinserttagmask_decode4659904&)*this).Xm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcinserttagmask_decode4659904&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcinserttagmask_decode4659904&)*this).Xd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcinserttagmask_decode4659904&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 861:
    {
      // Node 261
      // Node 262
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 0:
        {
          // Node 263
          // Node 264
          opcode = aarch64_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191&)*this).Xm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191&)*this).Xd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_decode4663191&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1131:
    {
      // Node 395
      // Node 396
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 12:
        {
          // Node 397
          // Node 398
          opcode = aarch64_aarch64_integer_pac_pacga_dp_2src_decode4662751;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_pac_pacga_dp_2src_decode4662751&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacga_dp_2src_decode4662751&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacga_dp_2src_decode4662751&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_pac_pacga_dp_2src_decode4662751&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 664:
    {
      // Node 424
      // Node 425
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 426
          // Node 427
          opcode = aarch64_aarch64_integer_tags_mcgettag_decode4661013;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcgettag_decode4661013&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcgettag_decode4661013&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcgettag_decode4661013&)*this).Xt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcgettag_decode4661013&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 152:
    {
      // Node 434
      // Node 435
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 436
          // Node 437
          opcode = aarch64_aarch64_integer_tags_mcsettag_decode4662156;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettag_decode4662156&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettag_decode4662156&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettag_decode4662156&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettag_decode4662156&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 470
          // Node 471
          opcode = aarch64_aarch64_integer_tags_mcsettagpost_decode4663072;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpost_decode4663072&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpost_decode4663072&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpost_decode4663072&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpost_decode4663072&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 472
          // Node 473
          opcode = aarch64_aarch64_integer_tags_mcsettagpre_decode4663726;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpre_decode4663726&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpre_decode4663726&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpre_decode4663726&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpre_decode4663726&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 408:
    {
      // Node 444
      // Node 445
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 446
          // Node 447
          opcode = aarch64_aarch64_integer_tags_mcsettagandzerodata_decode4662281;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodata_decode4662281&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodata_decode4662281&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodata_decode4662281&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodata_decode4662281&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 448
          // Node 449
          opcode = aarch64_aarch64_integer_tags_mcsettagandzerodatapost_decode4663163;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapost_decode4663163&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapost_decode4663163&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapost_decode4663163&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapost_decode4663163&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 450
          // Node 451
          opcode = aarch64_aarch64_integer_tags_mcsettagandzerodatapre_decode4660974;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapre_decode4660974&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapre_decode4660974&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapre_decode4660974&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagandzerodatapre_decode4660974&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1176:
    {
      // Node 454
      // Node 455
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 456
          // Node 457
          opcode = aarch64_aarch64_integer_tags_mcsettagpair_decode4662758;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpair_decode4662758&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpair_decode4662758&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpair_decode4662758&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpair_decode4662758&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 466
          // Node 467
          opcode = aarch64_aarch64_integer_tags_mcsettagpairpost_decode4662003;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpost_decode4662003&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpost_decode4662003&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpost_decode4662003&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpost_decode4662003&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 468
          // Node 469
          opcode = aarch64_aarch64_integer_tags_mcsettagpairpre_decode4660116;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpre_decode4660116&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpre_decode4660116&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpre_decode4660116&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairpre_decode4660116&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1432:
    {
      // Node 458
      // Node 459
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 460
          // Node 461
          opcode = aarch64_aarch64_integer_tags_mcsettagpairandzerodata_decode4661595;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodata_decode4661595&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodata_decode4661595&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodata_decode4661595&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodata_decode4661595&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 462
          // Node 463
          opcode = aarch64_aarch64_integer_tags_mcsettagpairandzerodatapost_decode4660126;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapost_decode4660126&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapost_decode4660126&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapost_decode4660126&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapost_decode4660126&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 464
          // Node 465
          opcode = aarch64_aarch64_integer_tags_mcsettagpairandzerodatapre_decode4663450;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapre_decode4663450&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapre_decode4663450&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapre_decode4663450&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_tags_mcsettagpairandzerodatapre_decode4663450&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1056:
    {
      // Node 664
      // Node 665
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 1:
        {
          // Node 666
          // Node 667
          opcode = aarch64_aarch64_memory_exclusive_single_decode4659962;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4659962&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4659962&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4659962&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4659962&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4659962&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 688
          // Node 689
          opcode = aarch64_aarch64_memory_exclusive_single_decode4662818;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4662818&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4662818&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4662818&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4662818&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4662818&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 32:
    {
      // Node 668
      // Node 669
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 670
          // Node 671
          opcode = aarch64_aarch64_memory_exclusive_single_decode4660118;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660118&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660118&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660118&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660118&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660118&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 678
          // Node 679
          opcode = aarch64_aarch64_memory_exclusive_single_decode4661421;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661421&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661421&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661421&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661421&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661421&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 34:
    {
      // Node 672
      // Node 673
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 674
          // Node 675
          opcode = aarch64_aarch64_memory_exclusive_single_decode4660782;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660782&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660782&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660782&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660782&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660782&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 676
          // Node 677
          opcode = aarch64_aarch64_memory_exclusive_single_decode4660927;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660927&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660927&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660927&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660927&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4660927&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1058:
    {
      // Node 694
      // Node 695
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 696
          // Node 697
          opcode = aarch64_aarch64_memory_exclusive_single_decode4663386;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663386&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663386&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663386&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663386&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663386&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 698
          // Node 699
          opcode = aarch64_aarch64_memory_exclusive_single_decode4663615;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663615&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663615&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663615&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663615&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663615&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1062:
    {
      // Node 709
      // Node 710
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 711
          // Node 712
          opcode = aarch64_aarch64_memory_ordered_decode4659830;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659830&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659830&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659830&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659830&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659830&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 735
          // Node 736
          opcode = aarch64_aarch64_memory_ordered_decode4663316;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663316&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663316&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663316&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663316&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663316&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 38:
    {
      // Node 713
      // Node 714
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 1:
        {
          // Node 715
          // Node 716
          opcode = aarch64_aarch64_memory_ordered_decode4659911;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659911&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659911&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659911&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659911&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4659911&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 721
          // Node 722
          opcode = aarch64_aarch64_memory_ordered_decode4660611;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660611&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660611&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660611&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660611&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660611&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 36:
    {
      // Node 717
      // Node 718
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 719
          // Node 720
          opcode = aarch64_aarch64_memory_ordered_decode4660053;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660053&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660053&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660053&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660053&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4660053&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 723
          // Node 724
          opcode = aarch64_aarch64_memory_ordered_decode4661081;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4661081&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4661081&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4661081&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4661081&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4661081&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1060:
    {
      // Node 739
      // Node 740
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 741
          // Node 742
          opcode = aarch64_aarch64_memory_ordered_decode4663419;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663419&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663419&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663419&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663419&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663419&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 743
          // Node 744
          opcode = aarch64_aarch64_memory_ordered_decode4663670;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663670&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663670&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663670&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663670&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663670&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 453:
    {
      // Node 749
      // Node 750
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 36:
        {
          // Node 751
          // Node 752
          opcode = aarch64_aarch64_memory_orderedrcpc_decode4662110;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662110&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662110&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662110&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662110&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      // Node 982
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 983
          // Node 984
          opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4661643;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661643&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661643&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661643&)*this).S = BITSEL(ir, 12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661643&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661643&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661643&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1477:
    {
      // Node 753
      // Node 754
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 36:
        {
          // Node 755
          // Node 756
          opcode = aarch64_aarch64_memory_orderedrcpc_decode4662823;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662823&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662823&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662823&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4662823&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      // Node 993
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 994
          // Node 995
          opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4663396;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663396&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663396&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663396&)*this).S = BITSEL(ir, 12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663396&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663396&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663396&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 818:
    {
      // Node 795
      // Node 796
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 797
          // Node 798
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4660899&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 304:
    {
      // Node 799
      // Node 800
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 801
          // Node 802
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661010&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1334:
    {
      // Node 803
      // Node 804
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 805
          // Node 806
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661179&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 816:
    {
      // Node 807
      // Node 808
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 809
          // Node 810
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661307&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 820:
    {
      // Node 811
      // Node 812
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 813
          // Node 814
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661414&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1332:
    {
      // Node 815
      // Node 816
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 817
          // Node 818
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4661634&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1844:
    {
      // Node 819
      // Node 820
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 821
          // Node 822
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662042&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 306:
    {
      // Node 823
      // Node 824
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 825
          // Node 826
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662454&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1328:
    {
      // Node 827
      // Node 828
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 829
          // Node 830
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662520&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1840:
    {
      // Node 831
      // Node 832
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 833
          // Node 834
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662643&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1330:
    {
      // Node 835
      // Node 836
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 837
          // Node 838
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4662976&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 308:
    {
      // Node 839
      // Node 840
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 841
          // Node 842
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663563&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 310:
    {
      // Node 843
      // Node 844
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 845
          // Node 846
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_lda_stl_memory_single_general_immediate_signed_offset_lda_stl__decode4663759&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1476:
    {
      // Node 861
      // Node 862
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 863
          // Node 864
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660932&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 901
          // Node 902
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662340&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 921
          // Node 922
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661513&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 945
          // Node 946
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662212&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 962:
    {
      // Node 865
      // Node 866
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 867
          // Node 868
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661950&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 895
          // Node 896
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660713&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 917
          // Node 918
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660022&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 933
          // Node 934
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4659896&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1472:
    {
      // Node 869
      // Node 870
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 871
          // Node 872
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4661955&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 891
          // Node 892
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660027&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 925
          // Node 926
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663036&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 939
          // Node 940
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4661350&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 452:
    {
      // Node 873
      // Node 874
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 875
          // Node 876
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4662062&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 893
          // Node 894
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4660397&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 927
          // Node 928
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663324&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 947
          // Node 948
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662718&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1986:
    {
      // Node 877
      // Node 878
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 879
          // Node 880
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663000&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 448:
    {
      // Node 881
      // Node 882
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 883
          // Node 884
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663334&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 897
          // Node 898
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4661564&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 915
          // Node 916
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4659960&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 941
          // Node 942
          opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662008&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 963:
    {
      // Node 971
      // Node 972
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 973
          // Node 974
          opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4660430;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660430&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660430&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660430&)*this).S = BITSEL(ir, 12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660430&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660430&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660430&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1473:
    {
      // Node 978
      // Node 979
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 980
          // Node 981
          opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4661209;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661209&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661209&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661209&)*this).S = BITSEL(ir, 12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661209&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661209&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661209&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 449:
    {
      // Node 989
      // Node 990
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 991
          // Node 992
          opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4662851;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4662851&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4662851&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4662851&)*this).S = BITSEL(ir, 12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4662851&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4662851&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4662851&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1987:
    {
      // Node 1000
      // Node 1001
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 1002
          // Node 1003
          opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4663642;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663642&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663642&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663642&)*this).S = BITSEL(ir, 12);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663642&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663642&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663642&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 348:
    {
      // Node 1154
      // Node 1155
      switch (UNSIGNED_BITS(ir, 4,0)) 
      {
      case 0:
        {
          // Node 1156
          opcode = aarch64_aarch64_system_exceptions_debug_breakpoint_decode4662991;
          length = 4;
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_breakpoint_decode4662991&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_breakpoint_decode4662991&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 349:
    {
      // Node 1157
      // Node 1158
      switch (UNSIGNED_BITS(ir, 4,0)) 
      {
      case 2:
        {
          // Node 1159
          opcode = aarch64_aarch64_system_exceptions_debug_exception_decode4661211;
          length = 4;
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4661211&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4661211&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 1160
          opcode = aarch64_aarch64_system_exceptions_debug_exception_decode4662084;
          length = 4;
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4662084&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4662084&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 1161
          opcode = aarch64_aarch64_system_exceptions_debug_exception_decode4663287;
          length = 4;
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4663287&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_exception_decode4663287&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 346:
    {
      // Node 1162
      // Node 1163
      switch (UNSIGNED_BITS(ir, 4,0)) 
      {
      case 0:
        {
          // Node 1164
          opcode = aarch64_aarch64_system_exceptions_debug_halt_decode4662594;
          length = 4;
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_halt_decode4662594&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
          ((aarch64_decode_aarch64_fmt_system_exceptions_debug_halt_decode4662594&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 344:
    {
      // Node 1165
      // Node 1166
      switch (UNSIGNED_BITS(ir, 4,0)) 
      {
      case 1:
        {
          // Node 1167
          opcode = aarch64_aarch64_system_exceptions_runtime_hvc_decode4662148;
          length = 4;
          ((aarch64_decode_aarch64_fmt_system_exceptions_runtime_hvc_decode4662148&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
          ((aarch64_decode_aarch64_fmt_system_exceptions_runtime_hvc_decode4662148&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3:
        {
          // Node 1168
          opcode = aarch64_aarch64_system_exceptions_runtime_smc_decode4663617;
          length = 4;
          ((aarch64_decode_aarch64_fmt_system_exceptions_runtime_smc_decode4663617&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
          ((aarch64_decode_aarch64_fmt_system_exceptions_runtime_smc_decode4663617&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 1169
          opcode = aarch64_aarch64_system_exceptions_runtime_svc_decode4662799;
          length = 4;
          ((aarch64_decode_aarch64_fmt_system_exceptions_runtime_svc_decode4662799&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
          ((aarch64_decode_aarch64_fmt_system_exceptions_runtime_svc_decode4662799&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 371:
    {
      // Node 2109
      // Node 2110
      opcode = aarch64_aarch64_vector_crypto_sha3_xar_decode4661470;
      length = 4;
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4661470&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4661470&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4661470&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4661470&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_vector_crypto_sha3_xar_decode4661470&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 976:
    {
      // Node 2111
      // Node 2112
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 0:
        {
          // Node 2113
          // Node 2114
          opcode = aarch64_aarch64_vector_crypto_sha3op_sha1hash_choose_decode4662141;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_choose_decode4662141&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_choose_decode4662141&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_choose_decode4662141&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_choose_decode4662141&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 8:
        {
          // Node 2115
          // Node 2116
          opcode = aarch64_aarch64_vector_crypto_sha3op_sha1hash_majority_decode4660158;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_majority_decode4660158&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_majority_decode4660158&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_majority_decode4660158&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_majority_decode4660158&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 16:
        {
          // Node 2117
          // Node 2118
          opcode = aarch64_aarch64_vector_crypto_sha3op_sha1hash_parity_decode4662360;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_parity_decode4662360&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_parity_decode4662360&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_parity_decode4662360&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1hash_parity_decode4662360&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 24:
        {
          // Node 2119
          // Node 2120
          opcode = aarch64_aarch64_vector_crypto_sha3op_sha1sched0_decode4661735;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1sched0_decode4661735&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1sched0_decode4661735&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1sched0_decode4661735&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha1sched0_decode4661735&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 20:
        {
          // Node 2121
          // Node 2122
          opcode = aarch64_aarch64_vector_crypto_sha3op_sha256hash_decode4660665;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256hash_decode4660665&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256hash_decode4660665&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256hash_decode4660665&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256hash_decode4660665&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 12:
        {
          // Node 2123
          // Node 2124
          opcode = aarch64_aarch64_vector_crypto_sha3op_sha256sched1_decode4660329;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256sched1_decode4660329&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256sched1_decode4660329&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256sched1_decode4660329&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_sha3op_sha256sched1_decode4660329&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 526:
    {
      // Node 2308
      // Node 2309
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 25:
        {
          // Node 2310
          // Node 2311
          opcode = aarch64_aarch64_vector_transfer_integer_insert_decode4661896;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_transfer_integer_insert_decode4661896&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_vector_transfer_integer_insert_decode4661896&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_transfer_integer_insert_decode4661896&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_transfer_integer_insert_decode4661896&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 1039:
    {
      // Node 2318
      // Node 2319
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 1:
        {
          // Node 2320
          // Node 2321
          opcode = aarch64_aarch64_vector_transfer_vector_cpydup_sisd_decode4663417;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_sisd_decode4663417&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_sisd_decode4663417&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_sisd_decode4663417&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_sisd_decode4663417&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 782:
    {
      // Node 2328
      // Node 2329
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 0:
        {
          // Node 2330
          // Node 2331
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 1:
            {
              // Node 2332
              // Node 2333
              opcode = aarch64_aarch64_vector_transfer_vector_insert_decode4663098;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_insert_decode4663098&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_insert_decode4663098&)*this).imm4 = UNSIGNED_BITS(ir, 14,11);
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_insert_decode4663098&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_insert_decode4663098&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_insert_decode4663098&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,22)) 
  {
  case 393:
    {
      // Node 422
      // Node 423
      opcode = aarch64_aarch64_integer_tags_mcaddtag_decode4659982;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_tags_mcaddtag_decode4659982&)*this).uimm6 = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcaddtag_decode4659982&)*this).op3 = UNSIGNED_BITS(ir, 15,14);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcaddtag_decode4659982&)*this).uimm4 = UNSIGNED_BITS(ir, 13,10);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcaddtag_decode4659982&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcaddtag_decode4659982&)*this).Xd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcaddtag_decode4659982&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 150:
    {
      // Node 438
      // Node 439
      opcode = aarch64_aarch64_integer_tags_mcsettaganddatapair_decode4659797;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapair_decode4659797&)*this).simm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapair_decode4659797&)*this).Xt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapair_decode4659797&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapair_decode4659797&)*this).Xt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapair_decode4659797&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 278:
    {
      // Node 440
      // Node 441
      opcode = aarch64_aarch64_integer_tags_mcsettaganddatapairpost_decode4663819;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpost_decode4663819&)*this).simm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpost_decode4663819&)*this).Xt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpost_decode4663819&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpost_decode4663819&)*this).Xt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpost_decode4663819&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 406:
    {
      // Node 442
      // Node 443
      opcode = aarch64_aarch64_integer_tags_mcsettaganddatapairpre_decode4660925;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpre_decode4660925&)*this).simm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpre_decode4660925&)*this).Xt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpre_decode4660925&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpre_decode4660925&)*this).Xt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsettaganddatapairpre_decode4660925&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 395:
    {
      // Node 474
      // Node 475
      opcode = aarch64_aarch64_integer_tags_mcsubtag_decode4661222;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsubtag_decode4661222&)*this).uimm6 = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsubtag_decode4661222&)*this).op3 = UNSIGNED_BITS(ir, 15,14);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsubtag_decode4661222&)*this).uimm4 = UNSIGNED_BITS(ir, 13,10);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsubtag_decode4661222&)*this).Xn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsubtag_decode4661222&)*this).Xd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_tags_mcsubtag_decode4661222&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 677:
    {
      // Node 761
      // Node 762
      opcode = aarch64_aarch64_memory_pair_general_offset_memory_pair_general_postidx__decode4659909;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4659909&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4659909&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4659909&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4659909&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4659909&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 681:
    {
      // Node 771
      // Node 772
      opcode = aarch64_aarch64_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4663784&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 685:
    {
      // Node 775
      // Node 776
      opcode = aarch64_aarch64_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661265&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 746:
    {
      // Node 951
      // Node 952
      opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660690&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 232:
    {
      // Node 955
      // Node 956
      opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661195&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 234:
    {
      // Node 961
      // Node 962
      opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662678&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 744:
    {
      // Node 963
      // Node 964
      opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663125&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 489:
    {
      // Node 967
      // Node 968
      opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663797&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 1001:
    {
      // Node 969
      // Node 970
      opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_unsigned__decode4663680&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 764:
    {
      // Node 1289
      // Node 1290
      switch (UNSIGNED_BITS(ir, 15,12)) 
      {
      case 9:
        {
          // Node 1291
          // Node 1292
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1293
              // Node 1294
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4659818&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 636:
    {
      // Node 1295
      // Node 1296
      switch (UNSIGNED_BITS(ir, 15,12)) 
      {
      case 9:
        {
          // Node 1297
          // Node 1298
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1299
              // Node 1300
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_sisd_decode4661877&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 2:
        {
          // Node 1377
          // Node 1378
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1379
              // Node 1380
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4660392&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 6:
        {
          // Node 1381
          // Node 1382
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1383
              // Node 1384
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_sisd_decode4661746&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 456:
    {
      // Node 2359
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 3113:
        {
          // Node 2095
          // Node 2096
          opcode = aarch64_aarch64_vector_crypto_aes_mix_decode4660264;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4660264&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4660264&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4660264&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3117:
        {
          // Node 2097
          // Node 2098
          opcode = aarch64_aarch64_vector_crypto_aes_mix_decode4662866;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4662866&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4662866&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_mix_decode4662866&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1065:
        {
          // Node 2099
          // Node 2100
          opcode = aarch64_aarch64_vector_crypto_aes_round_decode4660495;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4660495&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4660495&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4660495&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1069:
        {
          // Node 2101
          // Node 2102
          opcode = aarch64_aarch64_vector_crypto_aes_round_decode4661335;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4661335&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4661335&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_crypto_aes_round_decode4661335&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,23)) 
  {
  case 9:
    {
      // Node 483
      // Node 484
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 485
          // Node 486
          switch (UNSIGNED_BITS(ir, 14,10)) 
          {
          case 31:
            {
              // Node 487
              // Node 488
              opcode = aarch64_aarch64_memory_atomicops_cas_single_decode4660193;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4660193&)*this).L = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4660193&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4660193&)*this).o0 = BITSEL(ir, 15);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4660193&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4660193&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4660193&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 265:
    {
      // Node 489
      // Node 490
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 491
          // Node 492
          switch (UNSIGNED_BITS(ir, 14,10)) 
          {
          case 31:
            {
              // Node 493
              // Node 494
              opcode = aarch64_aarch64_memory_atomicops_cas_single_decode4662333;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4662333&)*this).L = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4662333&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4662333&)*this).o0 = BITSEL(ir, 15);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4662333&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4662333&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4662333&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 112:
    {
      // Node 562
      // Node 563
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 564
          // Node 565
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 16:
            {
              // Node 566
              // Node 567
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 568
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4659773;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659773&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659773&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659773&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659773&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 12:
            {
              // Node 569
              // Node 570
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 571
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4659926;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659926&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659926&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659926&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659926&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 4:
            {
              // Node 595
              // Node 596
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 597
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4660649;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660649&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660649&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660649&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660649&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 20:
            {
              // Node 601
              // Node 602
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 603
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4661130;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661130&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661130&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661130&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661130&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 28:
            {
              // Node 610
              // Node 611
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 612
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4661326;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661326&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661326&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661326&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661326&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 8:
            {
              // Node 613
              // Node 614
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 615
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4661555;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661555&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661555&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661555&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661555&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 0:
            {
              // Node 631
              // Node 632
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 633
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4661856;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661856&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661856&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661856&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661856&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 24:
            {
              // Node 637
              // Node 638
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 639
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4663086;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663086&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663086&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663086&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663086&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 368:
    {
      // Node 582
      // Node 583
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 584
          // Node 585
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 12:
            {
              // Node 586
              // Node 587
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 588
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4660362;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660362&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660362&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660362&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660362&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 16:
            {
              // Node 592
              // Node 593
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 594
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4660527;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660527&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660527&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660527&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660527&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 4:
            {
              // Node 598
              // Node 599
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 600
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4661098;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661098&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661098&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661098&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661098&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 8:
            {
              // Node 607
              // Node 608
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 609
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4661241;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661241&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661241&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661241&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661241&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 28:
            {
              // Node 619
              // Node 620
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 621
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4661690;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661690&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661690&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661690&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661690&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 20:
            {
              // Node 625
              // Node 626
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 627
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4661726;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661726&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661726&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661726&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661726&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 0:
            {
              // Node 634
              // Node 635
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 636
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4662436;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4662436&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4662436&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4662436&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4662436&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 24:
            {
              // Node 640
              // Node 641
              switch (UNSIGNED_BITS(ir, 4,0)) 
              {
              case 31:
                {
                  // Node 642
                  opcode = aarch64_aarch64_memory_atomicops_st_decode4663470;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663470&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663470&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663470&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663470&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 113:
    {
      // Node 855
      // Node 856
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 0:
        {
          // Node 857
          // Node 858
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 0:
            {
              // Node 859
              // Node 860
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660670&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 907
              // Node 908
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663622&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 923
              // Node 924
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4661610&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3:
            {
              // Node 935
              // Node 936
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660491&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 1:
        {
          // Node 985
          // Node 986
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 1:
            {
              // Node 987
              // Node 988
              opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4661920;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4661920&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 369:
    {
      // Node 885
      // Node 886
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 0:
        {
          // Node 887
          // Node 888
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 0:
            {
              // Node 889
              // Node 890
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4663465&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 899
              // Node 900
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662259&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 919
              // Node 920
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4660768&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3:
            {
              // Node 949
              // Node 950
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4663359&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 1:
        {
          // Node 1004
          // Node 1005
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 1:
            {
              // Node 1006
              // Node 1007
              opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4663747;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747&)*this).opc = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663747&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 373:
    {
      // Node 957
      // Node 958
      opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484&)*this).opc = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4661484&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 117:
    {
      // Node 965
      // Node 966
      opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489&)*this).opc = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4663489&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 319:
    {
      // Node 1313
      // Node 1314
      switch (UNSIGNED_BITS(ir, 15,12)) 
      {
      case 9:
        {
          // Node 1315
          // Node 1316
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1317
              // Node 1318
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4659955&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 6:
        {
          // Node 1393
          // Node 1394
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1395
              // Node 1396
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4660408&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 2:
        {
          // Node 1397
          // Node 1398
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1399
              // Node 1400
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_sisd_decode4663742&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 383:
    {
      // Node 1319
      // Node 1320
      switch (UNSIGNED_BITS(ir, 15,12)) 
      {
      case 9:
        {
          // Node 1321
          // Node 1322
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1323
              // Node 1324
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_sisd_decode4662562&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 271:
    {
      // Node 1687
      // Node 1688
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 157:
        {
          // Node 1955
          // Node 1956
          opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4663812&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 221:
        {
          // Node 1961
          // Node 1962
          opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_int_sisd_decode4661340;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661340&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661340&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661340&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661340&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 1689
          // Node 1690
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 55:
            {
              // Node 1691
              // Node 1692
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_sisd_decode4663549&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 63:
            {
              // Node 1731
              // Node 1732
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_recps_sisd_decode4661430;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_sisd_decode4661430&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_sisd_decode4661430&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_sisd_decode4661430&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_sisd_decode4661430&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_sisd_decode4661430&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 303:
    {
      // Node 1743
      // Node 1744
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 1545:
        {
          // Node 1821
          // Node 1822
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660378&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1549:
        {
          // Node 1827
          // Node 1828
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663655&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 185:
        {
          // Node 1831
          // Node 1832
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_sisd_decode4662424&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 253:
        {
          // Node 2067
          // Node 2068
          opcode = aarch64_aarch64_vector_arithmetic_unary_special_frecpx_decode4662670;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpx_decode4662670&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpx_decode4662670&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpx_decode4662670&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpx_decode4662670&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 221:
        {
          // Node 2073
          // Node 2074
          opcode = aarch64_aarch64_vector_arithmetic_unary_special_recip_float_sisd_decode4660787;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_sisd_decode4660787&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_sisd_decode4660787&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_sisd_decode4660787&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_sisd_decode4660787&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 1745
          // Node 1746
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 63:
            {
              // Node 1747
              // Node 1748
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_sisd_decode4661672&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 367:
    {
      // Node 1783
      // Node 1784
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 1549:
        {
          // Node 1823
          // Node 1824
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4660947&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1545:
        {
          // Node 1825
          // Node 1826
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_sisd_decode4663479&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 221:
        {
          // Node 2085
          // Node 2086
          opcode = aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_sisd_decode4663685&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 1785
          // Node 1786
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 23:
            {
              // Node 1787
              // Node 1788
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_sisd_decode4663486&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 335:
    {
      // Node 1951
      // Node 1952
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 157:
        {
          // Node 1953
          // Node 1954
          opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_sisd_decode4660753&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 221:
        {
          // Node 1963
          // Node 1964
          opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_int_sisd_decode4661822;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661822&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661822&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661822&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_sisd_decode4661822&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 181:
        {
          // Node 1991
          // Node 1992
          opcode = aarch64_aarch64_vector_arithmetic_unary_float_xtn_sisd_decode4660350;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_sisd_decode4660350&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_sisd_decode4660350&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_sisd_decode4660350&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_sisd_decode4660350&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 473:
        {
          // Node 2172
          // Node 2173
          opcode = aarch64_aarch64_vector_reduce_fpadd_sisd_decode4662713;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_reduce_fpadd_sisd_decode4662713&)*this).sz = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fpadd_sisd_decode4662713&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fpadd_sisd_decode4662713&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fpadd_sisd_decode4662713&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 428:
    {
      // Node 2360
      switch (UNSIGNED_BITS(ir, 22,10)) 
      {
      case 6080:
        {
          // Node 27
          // Node 28
          switch (UNSIGNED_BITS(ir, 4,0)) 
          {
          case 0:
            {
              // Node 29
              opcode = aarch64_aarch64_branch_unconditional_register_decode4661183;
              length = 4;
              ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4661183&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4661183&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 4032:
        {
          // Node 30
          // Node 31
          switch (UNSIGNED_BITS(ir, 4,0)) 
          {
          case 0:
            {
              // Node 32
              opcode = aarch64_aarch64_branch_unconditional_register_decode4661988;
              length = 4;
              ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4661988&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4661988&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 1984:
        {
          // Node 33
          // Node 34
          switch (UNSIGNED_BITS(ir, 4,0)) 
          {
          case 0:
            {
              // Node 35
              opcode = aarch64_aarch64_branch_unconditional_register_decode4662511;
              length = 4;
              ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662511&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662511&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 22,11)) 
      {
      case 2017:
        {
          // Node 38
          // Node 39
          switch (UNSIGNED_BITS(ir, 9,0)) 
          {
          case 1023:
            {
              // Node 40
              opcode = aarch64_aarch64_branch_unconditional_register_decode4663102;
              length = 4;
              ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4663102&)*this).M = BITSEL(ir, 10);
              ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4663102&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,24)) 
  {
  case 84:
    {
      // Node 6
      // Node 7
      switch ((ir & BIT_LSB(4)) >> 4)
      {
      case 0:
        {
          // Node 8
          // Node 9
          opcode = aarch64_aarch64_branch_conditional_cond_decode4661873;
          length = 4;
          ((aarch64_decode_aarch64_fmt_branch_conditional_cond_decode4661873&)*this).imm19 = UNSIGNED_BITS(ir, 23,5);
          ((aarch64_decode_aarch64_fmt_branch_conditional_cond_decode4661873&)*this).cond = UNSIGNED_BITS(ir, 3,0);
          ((aarch64_decode_aarch64_fmt_branch_conditional_cond_decode4661873&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 15:
    {
      // Node 41
      // Node 42
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 2305:
        {
          // Node 77
          // Node 78
          opcode = aarch64_aarch64_float_arithmetic_round_frint_decode4661146;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661146&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661146&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661146&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661146&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2433:
        {
          // Node 79
          // Node 80
          opcode = aarch64_aarch64_float_arithmetic_round_frint_decode4661406;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661406&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661406&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661406&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661406&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2337:
        {
          // Node 81
          // Node 82
          opcode = aarch64_aarch64_float_arithmetic_round_frint_decode4661851;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661851&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661851&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661851&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4661851&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2401:
        {
          // Node 83
          // Node 84
          opcode = aarch64_aarch64_float_arithmetic_round_frint_decode4662480;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662480&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662480&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662480&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662480&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2497:
        {
          // Node 85
          // Node 86
          opcode = aarch64_aarch64_float_arithmetic_round_frint_decode4662831;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662831&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662831&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662831&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662831&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2529:
        {
          // Node 87
          // Node 88
          opcode = aarch64_aarch64_float_arithmetic_round_frint_decode4662913;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662913&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662913&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662913&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4662913&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2369:
        {
          // Node 89
          // Node 90
          opcode = aarch64_aarch64_float_arithmetic_round_frint_decode4663447;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4663447&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4663447&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4663447&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_decode4663447&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 193:
        {
          // Node 91
          // Node 92
          opcode = aarch64_aarch64_float_arithmetic_unary_decode4660079;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660079&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660079&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660079&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660079&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 225:
        {
          // Node 93
          // Node 94
          opcode = aarch64_aarch64_float_arithmetic_unary_decode4660095;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660095&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660095&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660095&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4660095&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 161:
        {
          // Node 95
          // Node 96
          opcode = aarch64_aarch64_float_arithmetic_unary_decode4661230;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4661230&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4661230&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4661230&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4661230&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 129:
        {
          // Node 97
          // Node 98
          opcode = aarch64_aarch64_float_arithmetic_unary_decode4662972;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4662972&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4662972&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4662972&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_arithmetic_unary_decode4662972&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 21,17)) 
      {
      case 17:
        {
          // Node 124
          // Node 125
          switch (UNSIGNED_BITS(ir, 14,10)) 
          {
          case 1:
            {
              // Node 126
              // Node 127
              opcode = aarch64_aarch64_float_convert_fp_decode4661993;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_convert_fp_decode4661993&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_convert_fp_decode4661993&)*this).opc = UNSIGNED_BITS(ir, 16,15);
              ((aarch64_decode_aarch64_fmt_float_convert_fp_decode4661993&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_convert_fp_decode4661993&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_convert_fp_decode4661993&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 43
          // Node 44
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 37:
            {
              // Node 45
              // Node 46
              opcode = aarch64_aarch64_float_arithmetic_addsub_decode4659884;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4659884&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4659884&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4659884&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4659884&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4659884&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 33:
            {
              // Node 47
              // Node 48
              opcode = aarch64_aarch64_float_arithmetic_addsub_decode4660164;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4660164&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4660164&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4660164&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4660164&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_addsub_decode4660164&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 45:
            {
              // Node 49
              // Node 50
              opcode = aarch64_aarch64_float_arithmetic_maxmin_decode4661464;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4661464&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4661464&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4661464&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4661464&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4661464&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 41:
            {
              // Node 51
              // Node 52
              opcode = aarch64_aarch64_float_arithmetic_maxmin_decode4662587;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662587&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662587&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662587&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662587&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_maxmin_decode4662587&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 8:
            {
              // Node 67
              // Node 68
              opcode = aarch64_aarch64_float_arithmetic_mul_product_decode4661960;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4661960&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4661960&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4661960&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4661960&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4661960&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 40:
            {
              // Node 69
              // Node 70
              opcode = aarch64_aarch64_float_arithmetic_mul_product_decode4662459;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4662459&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4662459&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4662459&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4662459&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_product_decode4662459&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 106
              // Node 107
              switch ((ir & BIT_LSB(4)) >> 4)
              {
              case 0:
                {
                  // Node 108
                  // Node 109
                  switch (UNSIGNED_BITS(ir, 2,0)) 
                  {
                  case 0:
                    {
                      // Node 110
                      opcode = aarch64_aarch64_float_compare_uncond_decode4661696;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661696&)*this).typ = UNSIGNED_BITS(ir, 23,22);
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661696&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661696&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661696&)*this).opc = BITSEL(ir, 3);
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661696&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 1:
                {
                  // Node 111
                  // Node 112
                  switch (UNSIGNED_BITS(ir, 2,0)) 
                  {
                  case 0:
                    {
                      // Node 113
                      opcode = aarch64_aarch64_float_compare_uncond_decode4661978;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661978&)*this).typ = UNSIGNED_BITS(ir, 23,22);
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661978&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661978&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661978&)*this).opc = BITSEL(ir, 3);
                      ((aarch64_decode_aarch64_fmt_float_compare_uncond_decode4661978&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              }
              break;
            }
          }
          // Node 160
          switch (UNSIGNED_BITS(ir, 12,5)) 
          {
          case 32:
            {
              // Node 161
              // Node 162
              opcode = aarch64_aarch64_float_move_fp_imm_decode4661440;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_move_fp_imm_decode4661440&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_move_fp_imm_decode4661440&)*this).imm8 = UNSIGNED_BITS(ir, 20,13);
              ((aarch64_decode_aarch64_fmt_float_move_fp_imm_decode4661440&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_move_fp_imm_decode4661440&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 99
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 2:
            {
              // Node 100
              // Node 101
              switch ((ir & BIT_LSB(4)) >> 4)
              {
              case 0:
                {
                  // Node 102
                  // Node 103
                  opcode = aarch64_aarch64_float_compare_cond_decode4660204;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4660204&)*this).typ = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4660204&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4660204&)*this).cond = UNSIGNED_BITS(ir, 15,12);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4660204&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4660204&)*this).nzcv = UNSIGNED_BITS(ir, 3,0);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4660204&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 1:
                {
                  // Node 104
                  // Node 105
                  opcode = aarch64_aarch64_float_compare_cond_decode4662650;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4662650&)*this).typ = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4662650&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4662650&)*this).cond = UNSIGNED_BITS(ir, 15,12);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4662650&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4662650&)*this).nzcv = UNSIGNED_BITS(ir, 3,0);
                  ((aarch64_decode_aarch64_fmt_float_compare_cond_decode4662650&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 3:
            {
              // Node 163
              // Node 164
              opcode = aarch64_aarch64_float_move_fp_select_decode4663824;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_move_fp_select_decode4663824&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_move_fp_select_decode4663824&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_move_fp_select_decode4663824&)*this).cond = UNSIGNED_BITS(ir, 15,12);
              ((aarch64_decode_aarch64_fmt_float_move_fp_select_decode4663824&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_move_fp_select_decode4663824&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_move_fp_select_decode4663824&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 31:
    {
      // Node 53
      // Node 54
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 55
          // Node 56
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 0:
            {
              // Node 57
              // Node 58
              opcode = aarch64_aarch64_float_arithmetic_mul_addsub_decode4659975;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4659975&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4659975&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4659975&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4659975&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4659975&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4659975&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 63
              // Node 64
              opcode = aarch64_aarch64_float_arithmetic_mul_addsub_decode4661929;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4661929&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4661929&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4661929&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4661929&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4661929&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4661929&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 0:
        {
          // Node 59
          // Node 60
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 1:
            {
              // Node 61
              // Node 62
              opcode = aarch64_aarch64_float_arithmetic_mul_addsub_decode4660987;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4660987&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4660987&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4660987&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4660987&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4660987&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4660987&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 65
              // Node 66
              opcode = aarch64_aarch64_float_arithmetic_mul_addsub_decode4663122;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4663122&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4663122&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4663122&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4663122&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4663122&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_mul_addsub_decode4663122&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 120:
    {
      // Node 71
      // Node 72
      switch (UNSIGNED_BITS(ir, 21,17)) 
      {
      case 5:
        {
          // Node 73
          // Node 74
          switch (UNSIGNED_BITS(ir, 14,10)) 
          {
          case 1:
            {
              // Node 75
              // Node 76
              opcode = aarch64_aarch64_float_arithmetic_round_frint_32_64_decode4663342;
              length = 4;
              ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_32_64_decode4663342&)*this).typ = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_32_64_decode4663342&)*this).op = UNSIGNED_BITS(ir, 16,15);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_32_64_decode4663342&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_32_64_decode4663342&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_float_arithmetic_round_frint_32_64_decode4663342&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 184:
    {
      // Node 508
      // Node 509
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 510
          // Node 511
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 4:
            {
              // Node 512
              // Node 513
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4659842;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659842&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659842&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659842&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659842&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659842&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659842&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 522
              // Node 523
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4659972;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659972&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659972&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659972&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659972&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659972&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659972&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 20:
            {
              // Node 524
              // Node 525
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4660233;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660233&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660233&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660233&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660233&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660233&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660233&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 28:
            {
              // Node 526
              // Node 527
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4660279;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660279&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660279&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660279&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660279&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660279&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660279&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 24:
            {
              // Node 534
              // Node 535
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4661785;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661785&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661785&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661785&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661785&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661785&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661785&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 16:
            {
              // Node 546
              // Node 547
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4662846;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662846&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662846&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662846&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662846&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662846&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662846&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 12:
            {
              // Node 550
              // Node 551
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4663269;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663269&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663269&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663269&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663269&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663269&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663269&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 8:
            {
              // Node 556
              // Node 557
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4663632;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663632&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663632&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663632&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663632&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663632&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663632&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 32:
            {
              // Node 646
              // Node 647
              opcode = aarch64_aarch64_memory_atomicops_swp_decode4660004;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4660004&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4660004&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4660004&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4660004&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4660004&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4660004&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 56:
    {
      // Node 514
      // Node 515
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 516
          // Node 517
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 28:
            {
              // Node 518
              // Node 519
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4659856;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659856&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659856&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659856&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659856&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659856&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659856&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 20:
            {
              // Node 520
              // Node 521
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4659889;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659889&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659889&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659889&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659889&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659889&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659889&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 24:
            {
              // Node 530
              // Node 531
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4661270;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661270&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661270&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661270&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661270&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661270&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661270&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 16:
            {
              // Node 536
              // Node 537
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4662136;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662136&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662136&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662136&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662136&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662136&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662136&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 12:
            {
              // Node 544
              // Node 545
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4662804;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662804&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662804&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662804&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662804&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662804&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662804&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 8:
            {
              // Node 548
              // Node 549
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4662879;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662879&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662879&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662879&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662879&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662879&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662879&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 4:
            {
              // Node 554
              // Node 555
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4663401;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663401&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663401&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663401&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663401&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663401&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663401&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 558
              // Node 559
              opcode = aarch64_aarch64_memory_atomicops_ld_decode4663637;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663637&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663637&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663637&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663637&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663637&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663637&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 32:
            {
              // Node 650
              // Node 651
              opcode = aarch64_aarch64_memory_atomicops_swp_decode4662871;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662871&)*this).A = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662871&)*this).R = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662871&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662871&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662871&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662871&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 112:
    {
      // Node 700
      // Node 701
      opcode = aarch64_aarch64_memory_literal_general_decode4660239;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4660239&)*this).imm19 = UNSIGNED_BITS(ir, 23,5);
      ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4660239&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4660239&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 240:
    {
      // Node 704
      // Node 705
      opcode = aarch64_aarch64_memory_literal_general_decode4663539;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4663539&)*this).imm19 = UNSIGNED_BITS(ir, 23,5);
      ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4663539&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4663539&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 248:
    {
      // Node 909
      // Node 910
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 911
          // Node 912
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 1:
            {
              // Node 913
              // Node 914
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_pac_decode4662489;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489&)*this).M = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489&)*this).S = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489&)*this).W = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_pac_decode4662489&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 159:
    {
      // Node 1271
      // Node 1272
      switch (UNSIGNED_BITS(ir, 15,12)) 
      {
      case 13:
        {
          // Node 1273
          // Node 1274
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1275
              // Node 1276
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_double_sisd_decode4663668;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_sisd_decode4663668&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 6:
        {
          // Node 1333
          // Node 1334
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1335
              // Node 1336
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_high_sisd_decode4663391;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663391&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 7:
        {
          // Node 1337
          // Node 1338
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1339
              // Node 1340
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_high_sisd_decode4663831;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_sisd_decode4663831&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 3:
        {
          // Node 1365
          // Node 1366
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1367
              // Node 1368
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_sisd_decode4661294&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 191:
    {
      // Node 1409
      // Node 1410
      switch (UNSIGNED_BITS(ir, 15,12)) 
      {
      case 13:
        {
          // Node 1411
          // Node 1412
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1413
              // Node 1414
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660085&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 15:
        {
          // Node 1415
          // Node 1416
          switch ((ir & BIT_LSB(10)) >> 10)
          {
          case 0:
            {
              // Node 1417
              // Node 1418
              opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221&)*this).L = BITSEL(ir, 21);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221&)*this).M = BITSEL(ir, 20);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221&)*this).H = BITSEL(ir, 11);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_sisd_decode4660221&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 143:
    {
      // Node 2356
      switch (UNSIGNED_BITS(ir, 23,10)) 
      {
      case 11385:
        {
          // Node 1841
          // Node 1842
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4660500;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4660500&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4660500&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4660500&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 11389:
        {
          // Node 1847
          // Node 1848
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662841;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662841&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662841&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662841&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 10169:
        {
          // Node 1851
          // Node 1852
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode4663627;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode4663627&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode4663627&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_sisd_decode4663627&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 5753:
        {
          // Node 2011
          // Node 2012
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661361;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661361&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661361&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661361&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 13949:
        {
          // Node 2015
          // Node 2016
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662175;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662175&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662175&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662175&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 13945:
        {
          // Node 2017
          // Node 2018
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662531;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662531&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662531&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4662531&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 5757:
        {
          // Node 2023
          // Node 2024
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663807;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663807&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663807&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663807&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1949:
        {
          // Node 2029
          // Node 2030
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4660113;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4660113&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4660113&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4660113&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2013:
        {
          // Node 2037
          // Node 2038
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661445;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661445&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661445&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661445&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 10237:
        {
          // Node 2069
          // Node 2070
          opcode = aarch64_aarch64_vector_arithmetic_unary_special_frecpxfp16_decode4660511;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpxfp16_decode4660511&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpxfp16_decode4660511&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_frecpxfp16_decode4660511&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 10205:
        {
          // Node 2077
          // Node 2078
          opcode = aarch64_aarch64_vector_arithmetic_unary_special_recip_fp16_sisd_decode4660212;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_sisd_decode4660212&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_sisd_decode4660212&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_sisd_decode4660212&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 505:
        {
          // Node 2160
          // Node 2161
          opcode = aarch64_aarch64_vector_reduce_fp16max_sisd_decode4662961;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4662961&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4662961&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4662961&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 8697:
        {
          // Node 2162
          // Node 2163
          opcode = aarch64_aarch64_vector_reduce_fp16max_sisd_decode4663455;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4663455&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4663455&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_sisd_decode4663455&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 8601:
        {
          // Node 2168
          // Node 2169
          opcode = aarch64_aarch64_vector_reduce_fp16maxnm_sisd_decode4659916;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4659916&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4659916&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4659916&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 409:
        {
          // Node 2170
          // Node 2171
          opcode = aarch64_aarch64_vector_reduce_fp16maxnm_sisd_decode4663136;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4663136&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4663136&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_sisd_decode4663136&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 23,21)) 
      {
      case 1:
        {
          // Node 1517
          // Node 1518
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 17:
            {
              // Node 1519
              // Node 1520
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4660153&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 13:
            {
              // Node 1673
              // Node 1674
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_sisd_decode4660441&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 15:
            {
              // Node 1735
              // Node 1736
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_sisd_decode4662568&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 5:
        {
          // Node 1751
          // Node 1752
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 15:
            {
              // Node 1753
              // Node 1754
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_sisd_decode4663293&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch ((ir & BIT_LSB(23)) >> 23)
      {
      case 0:
        {
          // Node 1567
          // Node 1568
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 2829:
            {
              // Node 1931
              // Node 1932
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4660017&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2825:
            {
              // Node 1943
              // Node 1944
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662956&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1569
              // Node 1570
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 29:
                {
                  // Node 1571
                  // Node 1572
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4663234&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 1:
        {
          // Node 1933
          // Node 1934
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 2829:
            {
              // Node 1935
              // Node 1936
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661091&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2825:
            {
              // Node 1941
              // Node 1942
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4662898&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 2209
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 63:
            {
              // Node 2210
              // Node 2211
              opcode = aarch64_aarch64_vector_shift_conv_float_sisd_decode4662390;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4662390&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4662390&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4662390&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4662390&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4662390&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 15:
            {
              // Node 2218
              // Node 2219
              opcode = aarch64_aarch64_vector_shift_conv_int_sisd_decode4660199;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4660199&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4660199&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4660199&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4660199&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4660199&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 21:
            {
              // Node 2222
              // Node 2223
              opcode = aarch64_aarch64_vector_shift_left_sisd_decode4662659;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_left_sisd_decode4662659&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_sisd_decode4662659&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_sisd_decode4662659&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_sisd_decode4662659&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_sisd_decode4662659&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 54:
            {
              // Node 2242
              // Node 2243
              opcode = aarch64_aarch64_vector_shift_leftsat_sisd_decode4663736;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663736&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663736&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663736&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663736&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663736&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 6:
            {
              // Node 2260
              // Node 2261
              opcode = aarch64_aarch64_vector_shift_right_sisd_decode4660009;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660009&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660009&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660009&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660009&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660009&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 10:
            {
              // Node 2262
              // Node 2263
              opcode = aarch64_aarch64_vector_shift_right_sisd_decode4660893;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660893&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660893&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660893&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660893&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660893&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 2266
              // Node 2267
              opcode = aarch64_aarch64_vector_shift_right_sisd_decode4661579;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4661579&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4661579&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4661579&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4661579&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4661579&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 14:
            {
              // Node 2274
              // Node 2275
              opcode = aarch64_aarch64_vector_shift_right_sisd_decode4663695;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663695&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663695&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663695&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663695&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663695&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 39:
            {
              // Node 2296
              // Node 2297
              opcode = aarch64_aarch64_vector_shift_rightnarrow_uniform_sisd_decode4661069;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661069&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661069&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661069&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661069&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661069&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 37:
            {
              // Node 2298
              // Node 2299
              opcode = aarch64_aarch64_vector_shift_rightnarrow_uniform_sisd_decode4661527;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661527&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661527&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661527&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661527&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4661527&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      // Node 1247
      // Node 1248
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 225:
        {
          // Node 1805
          // Node 1806
          opcode = aarch64_aarch64_vector_arithmetic_unary_add_saturating_sisd_decode4660460;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4660460&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4660460&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4660460&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4660460&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 521:
        {
          // Node 1865
          // Node 1866
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661659&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 525:
        {
          // Node 1867
          // Node 1868
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4662264&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 169:
        {
          // Node 1871
          // Node 1872
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_sisd_decode4662377&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 233:
        {
          // Node 1887
          // Node 1888
          opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_int_sisd_decode4662856;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4662856&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4662856&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4662856&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4662856&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 241:
        {
          // Node 1897
          // Node 1898
          opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660626&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 149:
        {
          // Node 1905
          // Node 1906
          opcode = aarch64_aarch64_vector_arithmetic_unary_extract_sat_sisd_decode4660942;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4660942&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4660942&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4660942&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4660942&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 493:
        {
          // Node 2148
          // Node 2149
          opcode = aarch64_aarch64_vector_reduce_add_sisd_decode4663031;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_reduce_add_sisd_decode4663031&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_reduce_add_sisd_decode4663031&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_reduce_add_sisd_decode4663031&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_reduce_add_sisd_decode4663031&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 1249
          // Node 1250
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 17:
            {
              // Node 1251
              // Node 1252
              opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4660180&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 25:
            {
              // Node 1253
              // Node 1254
              opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_sisd_decode4662090&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 44:
            {
              // Node 1257
              // Node 1258
              opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_sisd_decode4661125&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 33:
            {
              // Node 1475
              // Node 1476
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659861&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3:
            {
              // Node 1489
              // Node 1490
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4662382&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 35:
            {
              // Node 1497
              // Node 1498
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4660486&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 51:
            {
              // Node 1581
              // Node 1582
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4659991&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 49:
            {
              // Node 1585
              // Node 1586
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4661935&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 27:
            {
              // Node 1727
              // Node 1728
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661086&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 23:
            {
              // Node 1757
              // Node 1758
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4660793;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660793&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660793&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660793&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660793&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660793&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 19:
            {
              // Node 1759
              // Node 1760
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4661201;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661201&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661201&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661201&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661201&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661201&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 21:
            {
              // Node 1763
              // Node 1764
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4661912;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661912&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661912&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661912&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661912&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661912&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 41:
            {
              // Node 1789
              // Node 1790
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663196&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 175:
    {
      // Node 2358
      switch (UNSIGNED_BITS(ir, 23,10)) 
      {
      case 11389:
        {
          // Node 1843
          // Node 1844
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662788;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662788&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662788&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662788&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 11385:
        {
          // Node 1845
          // Node 1846
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662793;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662793&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662793&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_sisd_decode4662793&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 13945:
        {
          // Node 2009
          // Node 2010
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4660074;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4660074&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4660074&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4660074&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 5757:
        {
          // Node 2013
          // Node 2014
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661397;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661397&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661397&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4661397&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 13949:
        {
          // Node 2019
          // Node 2020
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663264;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663264&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663264&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663264&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 5753:
        {
          // Node 2021
          // Node 2022
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663309;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663309&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663309&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_sisd_decode4663309&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1949:
        {
          // Node 2031
          // Node 2032
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4662689;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4662689&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4662689&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_sisd_decode4662689&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2013:
        {
          // Node 2039
          // Node 2040
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661667;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661667&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661667&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_sisd_decode4661667&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 10205:
        {
          // Node 2089
          // Node 2090
          opcode = aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode4660910;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode4660910&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode4660910&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_sisd_decode4660910&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 23,21)) 
      {
      case 5:
        {
          // Node 1513
          // Node 1514
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 19:
            {
              // Node 1515
              // Node 1516
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4659851&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 17:
            {
              // Node 1527
              // Node 1528
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4663114&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 5:
            {
              // Node 1773
              // Node 1774
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_sisd_decode4662862&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 1:
        {
          // Node 1521
          // Node 1522
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 17:
            {
              // Node 1523
              // Node 1524
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661303&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 19:
            {
              // Node 1525
              // Node 1526
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_sisd_decode4661544&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch ((ir & BIT_LSB(23)) >> 23)
      {
      case 1:
        {
          // Node 1551
          // Node 1552
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 2825:
            {
              // Node 1929
              // Node 1930
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4659948&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2829:
            {
              // Node 1939
              // Node 1940
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661740&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 505:
            {
              // Node 2180
              // Node 2181
              opcode = aarch64_aarch64_vector_reduce_fpmax_sisd_decode4661906;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661906&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661906&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661906&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661906&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 409:
            {
              // Node 2188
              // Node 2189
              opcode = aarch64_aarch64_vector_reduce_fpmaxnm_sisd_decode4663544;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4663544&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4663544&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4663544&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4663544&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1553
              // Node 1554
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 29:
                {
                  // Node 1555
                  // Node 1556
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4660064&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 31:
                {
                  // Node 1557
                  // Node 1558
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662121&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          // Node 2206
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 63:
            {
              // Node 2207
              // Node 2208
              opcode = aarch64_aarch64_vector_shift_conv_float_sisd_decode4660737;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4660737&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4660737&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4660737&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4660737&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_sisd_decode4660737&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 15:
            {
              // Node 2216
              // Node 2217
              opcode = aarch64_aarch64_vector_shift_conv_int_sisd_decode4659995;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4659995&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4659995&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4659995&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4659995&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_sisd_decode4659995&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 21:
            {
              // Node 2226
              // Node 2227
              opcode = aarch64_aarch64_vector_shift_leftinsert_sisd_decode4661220;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_sisd_decode4661220&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_sisd_decode4661220&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_sisd_decode4661220&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_sisd_decode4661220&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_sisd_decode4661220&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 50:
            {
              // Node 2238
              // Node 2239
              opcode = aarch64_aarch64_vector_shift_leftsat_sisd_decode4660585;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4660585&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4660585&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4660585&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4660585&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4660585&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 54:
            {
              // Node 2240
              // Node 2241
              opcode = aarch64_aarch64_vector_shift_leftsat_sisd_decode4663536;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663536&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663536&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663536&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663536&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_sisd_decode4663536&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 2264
              // Node 2265
              opcode = aarch64_aarch64_vector_shift_right_sisd_decode4660984;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660984&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660984&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660984&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660984&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4660984&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 14:
            {
              // Node 2268
              // Node 2269
              opcode = aarch64_aarch64_vector_shift_right_sisd_decode4662095;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662095&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662095&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662095&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662095&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662095&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 6:
            {
              // Node 2270
              // Node 2271
              opcode = aarch64_aarch64_vector_shift_right_sisd_decode4662908;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662908&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662908&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662908&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662908&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4662908&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 10:
            {
              // Node 2272
              // Node 2273
              opcode = aarch64_aarch64_vector_shift_right_sisd_decode4663597;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663597&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663597&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663597&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663597&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_sisd_decode4663597&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 5:
            {
              // Node 2278
              // Node 2279
              opcode = aarch64_aarch64_vector_shift_rightinsert_sisd_decode4662995;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_sisd_decode4662995&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_sisd_decode4662995&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_sisd_decode4662995&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_sisd_decode4662995&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_sisd_decode4662995&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 7:
            {
              // Node 2286
              // Node 2287
              opcode = aarch64_aarch64_vector_shift_rightnarrow_nonuniform_sisd_decode4660700;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660700&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660700&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660700&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660700&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_sisd_decode4660700&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 39:
            {
              // Node 2300
              // Node 2301
              opcode = aarch64_aarch64_vector_shift_rightnarrow_uniform_sisd_decode4662450;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662450&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662450&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662450&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662450&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662450&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 37:
            {
              // Node 2302
              // Node 2303
              opcode = aarch64_aarch64_vector_shift_rightnarrow_uniform_sisd_decode4662573;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662573&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662573&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662573&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662573&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_sisd_decode4662573&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 0:
        {
          // Node 1559
          // Node 1560
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 2829:
            {
              // Node 1937
              // Node 1938
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4661262&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2825:
            {
              // Node 1945
              // Node 1946
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_sisd_decode4663243&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 505:
            {
              // Node 2178
              // Node 2179
              opcode = aarch64_aarch64_vector_reduce_fpmax_sisd_decode4661135;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661135&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661135&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661135&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_sisd_decode4661135&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 409:
            {
              // Node 2186
              // Node 2187
              opcode = aarch64_aarch64_vector_reduce_fpmaxnm_sisd_decode4661626;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4661626&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4661626&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4661626&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_sisd_decode4661626&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1561
              // Node 1562
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 29:
                {
                  // Node 1563
                  // Node 1564
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662499&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 31:
                {
                  // Node 1565
                  // Node 1566
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_sisd_decode4662967&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      // Node 1477
      // Node 1478
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 225:
        {
          // Node 1807
          // Node 1808
          opcode = aarch64_aarch64_vector_arithmetic_unary_add_saturating_sisd_decode4662809;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4662809&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4662809&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4662809&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_sisd_decode4662809&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 521:
        {
          // Node 1861
          // Node 1862
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4659835&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 525:
        {
          // Node 1863
          // Node 1864
          opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_sisd_decode4661061&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 233:
        {
          // Node 1889
          // Node 1890
          opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_int_sisd_decode4663223;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4663223&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4663223&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4663223&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_sisd_decode4663223&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 241:
        {
          // Node 1895
          // Node 1896
          opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_sisd_decode4660355&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 149:
        {
          // Node 1907
          // Node 1908
          opcode = aarch64_aarch64_vector_arithmetic_unary_extract_sat_sisd_decode4663207;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4663207&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4663207&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4663207&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_sisd_decode4663207&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 165:
        {
          // Node 1911
          // Node 1912
          opcode = aarch64_aarch64_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131&)*this).size = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_sisd_decode4660131&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 1479
          // Node 1480
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 33:
            {
              // Node 1481
              // Node 1482
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_saturating_sisd_decode4659873&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3:
            {
              // Node 1491
              // Node 1492
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_sisd_decode4663690&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 35:
            {
              // Node 1499
              // Node 1500
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_sisd_decode4662623&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 49:
            {
              // Node 1583
              // Node 1584
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4660478&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 51:
            {
              // Node 1587
              // Node 1588
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_sisd_decode4663372&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 27:
            {
              // Node 1725
              // Node 1726
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_sisd_decode4661079&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 23:
            {
              // Node 1755
              // Node 1756
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4660309;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660309&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660309&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660309&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660309&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4660309&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 19:
            {
              // Node 1761
              // Node 1762
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4661257;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661257&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661257&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661257&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661257&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4661257&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 17:
            {
              // Node 1765
              // Node 1766
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4662505;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4662505&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4662505&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4662505&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4662505&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4662505&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 21:
            {
              // Node 1767
              // Node 1768
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_shift_sisd_decode4663213;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4663213&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4663213&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4663213&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4663213&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_shift_sisd_decode4663213&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 41:
            {
              // Node 1791
              // Node 1792
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_saturating_sisd_decode4663329&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 0:
        {
          // Node 1719
          // Node 1720
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 7:
            {
              // Node 1721
              // Node 1722
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663055&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 5:
            {
              // Node 1723
              // Node 1724
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_sisd_decode4663427&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,25)) 
  {
  case 107:
    {
      // Node 23
      // Node 24
      switch (UNSIGNED_BITS(ir, 23,11)) 
      {
      case 993:
        {
          // Node 25
          // Node 26
          opcode = aarch64_aarch64_branch_unconditional_register_decode4660339;
          length = 4;
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4660339&)*this).Z = BITSEL(ir, 24);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4660339&)*this).M = BITSEL(ir, 10);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4660339&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4660339&)*this).Rm = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4660339&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3041:
        {
          // Node 36
          // Node 37
          opcode = aarch64_aarch64_branch_unconditional_register_decode4662554;
          length = 4;
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662554&)*this).Z = BITSEL(ir, 24);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662554&)*this).M = BITSEL(ir, 10);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662554&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662554&)*this).Rm = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_branch_unconditional_register_decode4662554&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 31,26)) 
  {
  case 20:
    {
      // Node 19
      // Node 20
      opcode = aarch64_aarch64_branch_unconditional_immediate_decode4660600;
      length = 4;
      ((aarch64_decode_aarch64_fmt_branch_unconditional_immediate_decode4660600&)*this).imm26 = UNSIGNED_BITS(ir, 25,0);
      ((aarch64_decode_aarch64_fmt_branch_unconditional_immediate_decode4660600&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 52:
    {
      // Node 21
      // Node 22
      opcode = aarch64_aarch64_branch_unconditional_immediate_decode4662335;
      length = 4;
      ((aarch64_decode_aarch64_fmt_branch_unconditional_immediate_decode4662335&)*this).imm26 = UNSIGNED_BITS(ir, 25,0);
      ((aarch64_decode_aarch64_fmt_branch_unconditional_immediate_decode4662335&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  }
  switch ((ir & BIT_LSB(31)) >> 31)
  {
  case 1:
    {
      // Node 165
      // Node 495
      switch (UNSIGNED_BITS(ir, 29,21)) 
      {
      case 33:
        {
          // Node 652
          // Node 653
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 1:
            {
              // Node 654
              // Node 655
              opcode = aarch64_aarch64_memory_exclusive_pair_decode4660282;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660282&)*this).sz = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660282&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660282&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660282&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660282&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660282&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 660
              // Node 661
              opcode = aarch64_aarch64_memory_exclusive_pair_decode4660763;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660763&)*this).sz = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660763&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660763&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660763&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660763&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660763&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 35:
        {
          // Node 656
          // Node 657
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 0:
            {
              // Node 658
              // Node 659
              opcode = aarch64_aarch64_memory_exclusive_pair_decode4660708;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660708&)*this).sz = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660708&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660708&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660708&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660708&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4660708&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 662
              // Node 663
              opcode = aarch64_aarch64_memory_exclusive_pair_decode4662692;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4662692&)*this).sz = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4662692&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4662692&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4662692&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4662692&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_pair_decode4662692&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 32:
        {
          // Node 680
          // Node 681
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 1:
            {
              // Node 682
              // Node 683
              opcode = aarch64_aarch64_memory_exclusive_single_decode4661799;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661799&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661799&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661799&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661799&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661799&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661799&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 692
              // Node 693
              opcode = aarch64_aarch64_memory_exclusive_single_decode4663108;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663108&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663108&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663108&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663108&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663108&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663108&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 34:
        {
          // Node 684
          // Node 685
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 1:
            {
              // Node 686
              // Node 687
              opcode = aarch64_aarch64_memory_exclusive_single_decode4661995;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661995&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661995&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661995&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661995&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661995&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4661995&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 690
              // Node 691
              opcode = aarch64_aarch64_memory_exclusive_single_decode4663044;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663044&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663044&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663044&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663044&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663044&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_exclusive_single_decode4663044&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 36:
        {
          // Node 725
          // Node 726
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 1:
            {
              // Node 727
              // Node 728
              opcode = aarch64_aarch64_memory_ordered_decode4662032;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662032&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662032&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662032&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662032&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662032&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662032&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 729
              // Node 730
              opcode = aarch64_aarch64_memory_ordered_decode4662426;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662426&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662426&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662426&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662426&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662426&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4662426&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 38:
        {
          // Node 731
          // Node 732
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 1:
            {
              // Node 733
              // Node 734
              opcode = aarch64_aarch64_memory_ordered_decode4663182;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663182&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663182&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663182&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663182&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663182&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663182&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 737
              // Node 738
              opcode = aarch64_aarch64_memory_ordered_decode4663361;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663361&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663361&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663361&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663361&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663361&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_ordered_decode4663361&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 453:
        {
          // Node 745
          // Node 746
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 36:
            {
              // Node 747
              // Node 748
              opcode = aarch64_aarch64_memory_orderedrcpc_decode4660923;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4660923&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4660923&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4660923&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4660923&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_orderedrcpc_decode4660923&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 975
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 1:
            {
              // Node 976
              // Node 977
              opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4660435;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4660435&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 448:
        {
          // Node 847
          // Node 848
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 0:
            {
              // Node 849
              // Node 850
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660136&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 903
              // Node 904
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4662550&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 931
              // Node 932
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663660&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3:
            {
              // Node 943
              // Node 944
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4662026&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 452:
        {
          // Node 851
          // Node 852
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 0:
            {
              // Node 853
              // Node 854
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_normal_memory_single_general_immediate_signed_offset_normal__decode4660451&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 905
              // Node 906
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_offset_unpriv_memory_single_general_immediate_signed_offset_unpriv__decode4663274&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 929
              // Node 930
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_postidx_memory_single_general_immediate_signed_postidx__decode4663347&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3:
            {
              // Node 937
              // Node 938
              opcode = aarch64_aarch64_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_signed_preidx_memory_single_general_immediate_signed_postidx__decode4660819&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 449:
        {
          // Node 996
          // Node 997
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 1:
            {
              // Node 998
              // Node 999
              opcode = aarch64_aarch64_memory_single_general_register_memory_single_general_register__decode4663570;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570&)*this).size = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_general_register_memory_single_general_register__decode4663570&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,22)) 
      {
      case 232:
        {
          // Node 953
          // Node 954
          opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875&)*this).size = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4660875&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 234:
        {
          // Node 959
          // Node 960
          opcode = aarch64_aarch64_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021&)*this).size = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_general_immediate_unsigned_memory_single_general_immediate_signed_postidx__decode4662021&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,23)) 
      {
      case 9:
        {
          // Node 496
          // Node 497
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 498
              // Node 499
              switch (UNSIGNED_BITS(ir, 14,10)) 
              {
              case 31:
                {
                  // Node 500
                  // Node 501
                  opcode = aarch64_aarch64_memory_atomicops_cas_single_decode4663354;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354&)*this).L = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354&)*this).o0 = BITSEL(ir, 15);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_single_decode4663354&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 112:
        {
          // Node 572
          // Node 573
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 574
              // Node 575
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 20:
                {
                  // Node 576
                  // Node 577
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 578
                      opcode = aarch64_aarch64_memory_atomicops_st_decode4659967;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659967&)*this).size = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659967&)*this).R = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659967&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659967&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4659967&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 24:
                {
                  // Node 579
                  // Node 580
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 581
                      opcode = aarch64_aarch64_memory_atomicops_st_decode4660032;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660032&)*this).size = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660032&)*this).R = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660032&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660032&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660032&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 4:
                {
                  // Node 589
                  // Node 590
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 591
                      opcode = aarch64_aarch64_memory_atomicops_st_decode4660402;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660402&)*this).size = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660402&)*this).R = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660402&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660402&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4660402&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 28:
                {
                  // Node 604
                  // Node 605
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 606
                      opcode = aarch64_aarch64_memory_atomicops_st_decode4661151;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661151&)*this).size = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661151&)*this).R = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661151&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661151&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661151&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 12:
                {
                  // Node 616
                  // Node 617
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 618
                      opcode = aarch64_aarch64_memory_atomicops_st_decode4661654;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661654&)*this).size = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661654&)*this).R = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661654&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661654&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661654&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 8:
                {
                  // Node 622
                  // Node 623
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 624
                      opcode = aarch64_aarch64_memory_atomicops_st_decode4661709;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661709&)*this).size = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661709&)*this).R = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661709&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661709&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661709&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 16:
                {
                  // Node 628
                  // Node 629
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 630
                      opcode = aarch64_aarch64_memory_atomicops_st_decode4661840;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661840&)*this).size = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661840&)*this).R = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661840&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661840&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4661840&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 0:
                {
                  // Node 643
                  // Node 644
                  switch (UNSIGNED_BITS(ir, 4,0)) 
                  {
                  case 31:
                    {
                      // Node 645
                      opcode = aarch64_aarch64_memory_atomicops_st_decode4663768;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663768&)*this).size = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663768&)*this).R = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663768&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663768&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_memory_atomicops_st_decode4663768&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,24)) 
      {
      case 56:
        {
          // Node 502
          // Node 503
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 504
              // Node 505
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 16:
                {
                  // Node 506
                  // Node 507
                  opcode = aarch64_aarch64_memory_atomicops_ld_decode4659787;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4659787&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 0:
                {
                  // Node 528
                  // Node 529
                  opcode = aarch64_aarch64_memory_atomicops_ld_decode4660829;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4660829&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 24:
                {
                  // Node 532
                  // Node 533
                  opcode = aarch64_aarch64_memory_atomicops_ld_decode4661392;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4661392&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 20:
                {
                  // Node 538
                  // Node 539
                  opcode = aarch64_aarch64_memory_atomicops_ld_decode4662278;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662278&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 4:
                {
                  // Node 540
                  // Node 541
                  opcode = aarch64_aarch64_memory_atomicops_ld_decode4662618;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662618&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 28:
                {
                  // Node 542
                  // Node 543
                  opcode = aarch64_aarch64_memory_atomicops_ld_decode4662655;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4662655&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 12:
                {
                  // Node 552
                  // Node 553
                  opcode = aarch64_aarch64_memory_atomicops_ld_decode4663302;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663302&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 8:
                {
                  // Node 560
                  // Node 561
                  opcode = aarch64_aarch64_memory_atomicops_ld_decode4663846;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_ld_decode4663846&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 32:
                {
                  // Node 648
                  // Node 649
                  opcode = aarch64_aarch64_memory_atomicops_swp_decode4662192;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192&)*this).size = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192&)*this).A = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192&)*this).R = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_swp_decode4662192&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      // Node 166
      switch (UNSIGNED_BITS(ir, 28,24)) 
      {
      case 1:
        {
          // Node 167
          // Node 168
          opcode = aarch64_aarch64_integer_arithmetic_address_pcrel_decode4661401;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4661401&)*this).immlo = UNSIGNED_BITS(ir, 30,29);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4661401&)*this).immhi = UNSIGNED_BITS(ir, 23,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4661401&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4661401&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 0:
    {
      // Node 169
      // Node 476
      switch (UNSIGNED_BITS(ir, 29,10)) 
      {
      case 767613:
        {
          // Node 1993
          // Node 1994
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660303&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 235133:
        {
          // Node 1995
          // Node 1996
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4660809&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 759417:
        {
          // Node 1997
          // Node 1998
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661278&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 767609:
        {
          // Node 1999
          // Node 2000
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4661378&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 759421:
        {
          // Node 2001
          // Node 2002
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4662771&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 243325:
        {
          // Node 2003
          // Node 2004
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663412&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 235129:
        {
          // Node 2005
          // Node 2006
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663442&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 243321:
        {
          // Node 2007
          // Node 2008
          opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773;
          length = 4;
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_bulk_simd_decode4663773&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,12)) 
      {
      case 24578:
        {
          // Node 1040
          // Node 1041
          opcode = aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551&)*this).size = UNSIGNED_BITS(ir, 11,10);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4660551&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 25601:
        {
          // Node 1042
          // Node 1043
          opcode = aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022&)*this).size = UNSIGNED_BITS(ir, 11,10);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661022&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 24576:
        {
          // Node 1044
          // Node 1045
          opcode = aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425&)*this).size = UNSIGNED_BITS(ir, 11,10);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661425&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 25602:
        {
          // Node 1046
          // Node 1047
          opcode = aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630&)*this).size = UNSIGNED_BITS(ir, 11,10);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4661630&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 25600:
        {
          // Node 1048
          // Node 1049
          opcode = aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386&)*this).size = UNSIGNED_BITS(ir, 11,10);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662386&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 24577:
        {
          // Node 1054
          // Node 1055
          opcode = aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167&)*this).Q = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167&)*this).size = UNSIGNED_BITS(ir, 11,10);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663167&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,16)) 
      {
      case 1536:
        {
          // Node 1050
          // Node 1051
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1052
              // Node 1053
              opcode = aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484&)*this).opcode_part2 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484&)*this).opcode = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4662484&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 1600:
        {
          // Node 1056
          // Node 1057
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1058
              // Node 1059
              opcode = aarch64_aarch64_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650&)*this).opcode_part2 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650&)*this).opcode = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_nowb_memory_vector_multiple_nowb__decode4663650&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 5664:
        {
          // Node 1088
          // Node 1089
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 0:
            {
              // Node 1090
              // Node 1091
              opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660593&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 1108
              // Node 1109
              opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662699&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 5632:
        {
          // Node 1102
          // Node 1103
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1104
              // Node 1105
              opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661730&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 1110
              // Node 1111
              opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662925&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,19)) 
      {
      case 30:
        {
          // Node 2125
          // Node 2126
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 63:
            {
              // Node 2127
              // Node 2128
              opcode = aarch64_aarch64_vector_fp16_movi_decode4659795;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).a = BITSEL(ir, 18);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).b = BITSEL(ir, 17);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).c = BITSEL(ir, 16);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).d = BITSEL(ir, 9);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).e = BITSEL(ir, 8);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).f = BITSEL(ir, 7);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).g = BITSEL(ir, 6);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).h = BITSEL(ir, 5);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_fp16_movi_decode4659795&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 2133
          switch (UNSIGNED_BITS(ir, 12,10)) 
          {
          case 5:
            {
              // Node 2134
              // Node 2135
              opcode = aarch64_aarch64_vector_logical_decode4660044;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).a = BITSEL(ir, 18);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).b = BITSEL(ir, 17);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).c = BITSEL(ir, 16);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).cmode_part1 = UNSIGNED_BITS(ir, 15,13);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).d = BITSEL(ir, 9);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).e = BITSEL(ir, 8);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).f = BITSEL(ir, 7);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).g = BITSEL(ir, 6);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).h = BITSEL(ir, 5);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4660044&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 1054:
        {
          // Node 2139
          // Node 2143
          switch (UNSIGNED_BITS(ir, 12,10)) 
          {
          case 5:
            {
              // Node 2144
              // Node 2145
              opcode = aarch64_aarch64_vector_logical_decode4663664;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).a = BITSEL(ir, 18);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).b = BITSEL(ir, 17);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).c = BITSEL(ir, 16);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).cmode_part1 = UNSIGNED_BITS(ir, 15,13);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).d = BITSEL(ir, 9);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).e = BITSEL(ir, 8);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).f = BITSEL(ir, 7);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).g = BITSEL(ir, 6);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).h = BITSEL(ir, 5);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4663664&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 2140
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 1:
            {
              // Node 2141
              // Node 2142
              opcode = aarch64_aarch64_vector_logical_decode4662766;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).a = BITSEL(ir, 18);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).b = BITSEL(ir, 17);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).c = BITSEL(ir, 16);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).cmode = UNSIGNED_BITS(ir, 15,12);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).d = BITSEL(ir, 9);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).e = BITSEL(ir, 8);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).f = BITSEL(ir, 7);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).g = BITSEL(ir, 6);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).h = BITSEL(ir, 5);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662766&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,21)) 
      {
      case 306:
        {
          // Node 1060
          // Node 1061
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 0:
            {
              // Node 1062
              // Node 1063
              opcode = aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4659999&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 1073
              // Node 1074
              opcode = aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661891&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 1078
              // Node 1079
              opcode = aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663106&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 1068
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1069
              // Node 1070
              opcode = aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858&)*this).opcode_part2 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858&)*this).opcode = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660858&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 304:
        {
          // Node 1064
          // Node 1065
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 2:
            {
              // Node 1066
              // Node 1067
              opcode = aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4660343&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 1071
              // Node 1072
              opcode = aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4661449&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 1080
              // Node 1081
              opcode = aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4663474&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 1075
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1076
              // Node 1077
              opcode = aarch64_aarch64_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493&)*this).opcode_part2 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493&)*this).opcode = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_multiple_postinc_memory_vector_multiple_nowb__decode4662493&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 434:
        {
          // Node 1114
          // Node 1115
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 14:
            {
              // Node 1116
              // Node 1117
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4659864&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 6:
            {
              // Node 1134
              // Node 1135
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661835&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 1141
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1142
              // Node 1143
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663319&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 1144
              // Node 1145
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663506&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 433:
        {
          // Node 1118
          // Node 1119
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1120
              // Node 1121
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4660883&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 1122
              // Node 1123
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661297&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 432:
        {
          // Node 1124
          // Node 1125
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1126
              // Node 1127
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661458&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 1128
              // Node 1129
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661662&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 435:
        {
          // Node 1130
          // Node 1131
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 14:
            {
              // Node 1132
              // Node 1133
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4661760&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 6:
            {
              // Node 1139
              // Node 1140
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663058&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 1136
          switch ((ir & BIT_LSB(13)) >> 13)
          {
          case 1:
            {
              // Node 1137
              // Node 1138
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4662695&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 0:
            {
              // Node 1146
              // Node 1147
              opcode = aarch64_aarch64_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_postinc_memory_vector_single_nowb__decode4663776&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 113:
        {
          // Node 1447
          // Node 1448
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 5:
            {
              // Node 1449
              // Node 1450
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp16_decode4660854;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4660854&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4660854&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4660854&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4660854&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4660854&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 17:
            {
              // Node 1507
              // Node 1508
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4662293&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 49:
            {
              // Node 1603
              // Node 1604
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_logical_andorr_decode4661345;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4661345&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4661345&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4661345&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4661345&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4661345&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 7:
            {
              // Node 1629
              // Node 1630
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4662705&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 1639
              // Node 1640
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663613&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 13:
            {
              // Node 1671
              // Node 1672
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_extended_simd_decode4661810&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 9:
            {
              // Node 1677
              // Node 1678
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4661550&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 15:
            {
              // Node 1733
              // Node 1734
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recpsfp16_simd_decode4660506&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 369:
        {
          // Node 1451
          // Node 1452
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 5:
            {
              // Node 1453
              // Node 1454
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp16_decode4661141;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4661141&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4661141&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4661141&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4661141&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp16_decode4661141&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 17:
            {
              // Node 1509
              // Node 1510
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663131&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 19:
            {
              // Node 1511
              // Node 1512
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4663592&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 15:
            {
              // Node 1597
              // Node 1598
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_divfp16_decode4660971;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_divfp16_decode4660971&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_divfp16_decode4660971&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_divfp16_decode4660971&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_divfp16_decode4660971&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_divfp16_decode4660971&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 49:
            {
              // Node 1621
              // Node 1622
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662197&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 7:
            {
              // Node 1631
              // Node 1632
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4663280&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 1633
              // Node 1634
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4660617&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 13:
            {
              // Node 1679
              // Node 1680
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_product_decode4660522&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 119:
        {
          // Node 1605
          // Node 1606
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 49:
            {
              // Node 1607
              // Node 1608
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_logical_andorr_decode4663314;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663314&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663314&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663314&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663314&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663314&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 115:
        {
          // Node 1609
          // Node 1610
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 49:
            {
              // Node 1611
              // Node 1612
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_logical_andorr_decode4663460;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663460&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663460&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663460&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663460&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4663460&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 375:
        {
          // Node 1613
          // Node 1614
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 49:
            {
              // Node 1615
              // Node 1616
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4660465&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 371:
        {
          // Node 1617
          // Node 1618
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 49:
            {
              // Node 1619
              // Node 1620
              opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4661776&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 14:
        {
          // Node 2304
          // Node 2305
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 17:
            {
              // Node 2306
              // Node 2307
              opcode = aarch64_aarch64_vector_transfer_integer_dup_decode4659921;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_dup_decode4659921&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_dup_decode4659921&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_dup_decode4659921&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_dup_decode4659921&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_dup_decode4659921&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 19:
            {
              // Node 2312
              // Node 2313
              opcode = aarch64_aarch64_vector_transfer_integer_move_signed_decode4662984;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_signed_decode4662984&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_signed_decode4662984&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_signed_decode4662984&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_signed_decode4662984&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_signed_decode4662984&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 23:
            {
              // Node 2314
              // Node 2315
              opcode = aarch64_aarch64_vector_transfer_integer_move_unsigned_decode4660293;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_unsigned_decode4660293&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_unsigned_decode4660293&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_unsigned_decode4660293&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_unsigned_decode4660293&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_transfer_integer_move_unsigned_decode4660293&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1:
            {
              // Node 2316
              // Node 2317
              opcode = aarch64_aarch64_vector_transfer_vector_cpydup_simd_decode4660598;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_simd_decode4660598&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_simd_decode4660598&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_simd_decode4660598&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_simd_decode4660598&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_transfer_vector_cpydup_simd_decode4660598&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 232:
        {
          // Node 2322
          // Node 2323
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 0:
            {
              // Node 2324
              // Node 2325
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 2326
                  // Node 2327
                  opcode = aarch64_aarch64_vector_transfer_vector_extract_decode4663603;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_extract_decode4663603&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_extract_decode4663603&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_extract_decode4663603&)*this).imm4 = UNSIGNED_BITS(ir, 14,11);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_extract_decode4663603&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_extract_decode4663603&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_extract_decode4663603&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 224:
        {
          // Node 2348
          // Node 2349
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 0:
            {
              // Node 2350
              // Node 2351
              switch (UNSIGNED_BITS(ir, 12,10)) 
              {
              case 0:
                {
                  // Node 2352
                  // Node 2353
                  opcode = aarch64_aarch64_vector_transfer_vector_table_decode4662068;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4662068&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4662068&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4662068&)*this).len = UNSIGNED_BITS(ir, 14,13);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4662068&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4662068&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4662068&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 4:
                {
                  // Node 2354
                  // Node 2355
                  opcode = aarch64_aarch64_vector_transfer_vector_table_decode4663188;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4663188&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4663188&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4663188&)*this).len = UNSIGNED_BITS(ir, 14,13);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4663188&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4663188&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_table_decode4663188&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 112:
        {
          // Node 2363
          switch (UNSIGNED_BITS(ir, 20,10)) 
          {
          case 1949:
            {
              // Node 2027
              // Node 2028
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4661419&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2013:
            {
              // Node 2033
              // Node 2034
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4662732&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1657:
            {
              // Node 2045
              // Node 2046
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4661251;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661251&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661251&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661251&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661251&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1661:
            {
              // Node 2047
              // Node 2048
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4661757;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661757&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661757&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661757&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4661757&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 889:
            {
              // Node 2158
              // Node 2159
              opcode = aarch64_aarch64_vector_reduce_fp16max_simd_decode4661074;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4661074&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4661074&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4661074&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4661074&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 793:
            {
              // Node 2164
              // Node 2165
              opcode = aarch64_aarch64_vector_reduce_fp16maxnm_simd_decode4660108;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4660108&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4660108&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4660108&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4660108&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 368:
        {
          // Node 2364
          switch (UNSIGNED_BITS(ir, 20,10)) 
          {
          case 1949:
            {
              // Node 2025
              // Node 2026
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_float_tieaway_simd_decode4660718&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2013:
            {
              // Node 2035
              // Node 2036
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_conv_int_simd_decode4663789&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1657:
            {
              // Node 2041
              // Node 2042
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4659878;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4659878&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4659878&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4659878&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4659878&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1661:
            {
              // Node 2053
              // Node 2054
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4663248;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663248&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663248&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663248&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663248&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 209:
            {
              // Node 2055
              // Node 2056
              opcode = aarch64_aarch64_vector_arithmetic_unary_not_decode4660992;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_not_decode4660992&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_not_decode4660992&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_not_decode4660992&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_not_decode4660992&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,22)) 
      {
      case 248:
        {
          // Node 1277
          // Node 1278
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 9:
            {
              // Node 1279
              // Node 1280
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1281
                  // Node 1282
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4662046&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 120:
        {
          // Node 1283
          // Node 1284
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 9:
            {
              // Node 1285
              // Node 1286
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1287
                  // Node 1288
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp16_simd_decode4663338&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 2:
            {
              // Node 1369
              // Node 1370
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1371
                  // Node 1372
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4661769&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 6:
            {
              // Node 1373
              // Node 1374
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1375
                  // Node 1376
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp16_simd_decode4663712&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 58:
        {
          // Node 2361
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 3197:
            {
              // Node 1833
              // Node 1834
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660569&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3193:
            {
              // Node 1839
              // Node 1840
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4661685&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1977:
            {
              // Node 1849
              // Node 1850
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_lessthan_simd_decode4660705&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2041:
            {
              // Node 1879
              // Node 1880
              opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_fp16_decode4660470;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4660470&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4660470&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4660470&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4660470&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1657:
            {
              // Node 2043
              // Node 2044
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4660824;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4660824&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4660824&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4660824&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4660824&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1661:
            {
              // Node 2051
              // Node 2052
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4663091;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663091&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663091&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663091&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4663091&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2013:
            {
              // Node 2075
              // Node 2076
              opcode = aarch64_aarch64_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_fp16_simd_decode4662252&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 889:
            {
              // Node 2156
              // Node 2157
              opcode = aarch64_aarch64_vector_reduce_fp16max_simd_decode4660037;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4660037&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4660037&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4660037&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16max_simd_decode4660037&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 793:
            {
              // Node 2166
              // Node 2167
              opcode = aarch64_aarch64_vector_reduce_fp16maxnm_simd_decode4662749;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4662749&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4662749&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4662749&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fp16maxnm_simd_decode4662749&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1599
              // Node 1600
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 49:
                {
                  // Node 1601
                  // Node 1602
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_logical_andorr_decode4659823;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4659823&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4659823&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4659823&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4659823&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_andorr_decode4659823&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 7:
                {
                  // Node 1627
                  // Node 1628
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4661356&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 1:
                {
                  // Node 1635
                  // Node 1636
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4662082&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 9:
                {
                  // Node 1675
                  // Node 1676
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp16_fused_decode4660533&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 15:
                {
                  // Node 1749
                  // Node 1750
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrtsfp16_simd_decode4660335&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 5:
                {
                  // Node 1769
                  // Node 1770
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4659814&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 186:
        {
          // Node 2362
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 3197:
            {
              // Node 1835
              // Node 1836
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660631&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3193:
            {
              // Node 1837
              // Node 1838
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_fp16_bulk_simd_decode4660636&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2041:
            {
              // Node 1881
              // Node 1882
              opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_fp16_decode4661115;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4661115&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4661115&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4661115&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_fp16_decode4661115&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1661:
            {
              // Node 2049
              // Node 2050
              opcode = aarch64_aarch64_vector_arithmetic_unary_fp16_round_decode4662325;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4662325&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4662325&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4662325&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_fp16_round_decode4662325&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 209:
            {
              // Node 2057
              // Node 2058
              opcode = aarch64_aarch64_vector_arithmetic_unary_rbit_decode4662126;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rbit_decode4662126&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rbit_decode4662126&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rbit_decode4662126&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rbit_decode4662126&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2013:
            {
              // Node 2087
              // Node 2088
              opcode = aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_fp16_simd_decode4662146&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2045:
            {
              // Node 2093
              // Node 2094
              opcode = aarch64_aarch64_vector_arithmetic_unary_special_sqrtfp16_decode4662238;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtfp16_decode4662238&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtfp16_decode4662238&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtfp16_decode4662238&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtfp16_decode4662238&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1501
              // Node 1502
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 17:
                {
                  // Node 1503
                  // Node 1504
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661056&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 19:
                {
                  // Node 1505
                  // Node 1506
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp16_simd_decode4661236&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 49:
                {
                  // Node 1623
                  // Node 1624
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_logical_bsleor_decode4662890&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 7:
                {
                  // Node 1625
                  // Node 1626
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_1985_decode4660743&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 1:
                {
                  // Node 1637
                  // Node 1638
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp16_2008_decode4663576&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 5:
                {
                  // Node 1771
                  // Node 1772
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp16_simd_decode4660191&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 89:
        {
          // Node 2365
          switch (UNSIGNED_BITS(ir, 21,12)) 
          {
          case 526:
            {
              // Node 1082
              // Node 1083
              opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4659943&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 14:
            {
              // Node 1092
              // Node 1093
              opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660760&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 518:
            {
              // Node 1094
              // Node 1095
              opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661410&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 6:
            {
              // Node 1096
              // Node 1097
              opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522&)*this).size = UNSIGNED_BITS(ir, 11,10);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661522&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch (UNSIGNED_BITS(ir, 21,16)) 
          {
          case 0:
            {
              // Node 1084
              // Node 1085
              switch ((ir & BIT_LSB(13)) >> 13)
              {
              case 0:
                {
                  // Node 1086
                  // Node 1087
                  opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455&)*this).S = BITSEL(ir, 12);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455&)*this).size = UNSIGNED_BITS(ir, 11,10);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4660455&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 1:
                {
                  // Node 1106
                  // Node 1107
                  opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331&)*this).S = BITSEL(ir, 12);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331&)*this).size = UNSIGNED_BITS(ir, 11,10);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4662331&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 32:
            {
              // Node 1098
              // Node 1099
              switch ((ir & BIT_LSB(13)) >> 13)
              {
              case 0:
                {
                  // Node 1100
                  // Node 1101
                  opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559&)*this).S = BITSEL(ir, 12);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559&)*this).size = UNSIGNED_BITS(ir, 11,10);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4661559&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 1:
                {
                  // Node 1112
                  // Node 1113
                  opcode = aarch64_aarch64_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646&)*this).opcode_part1 = UNSIGNED_BITS(ir, 15,14);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646&)*this).S = BITSEL(ir, 12);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646&)*this).size = UNSIGNED_BITS(ir, 11,10);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_vector_single_nowb_memory_vector_single_nowb__decode4663646&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,23)) 
      {
      case 8:
        {
          // Node 477
          // Node 478
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 479
              // Node 480
              switch (UNSIGNED_BITS(ir, 14,10)) 
              {
              case 31:
                {
                  // Node 481
                  // Node 482
                  opcode = aarch64_aarch64_memory_atomicops_cas_pair_decode4661064;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064&)*this).sz = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064&)*this).L = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064&)*this).Rs = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064&)*this).o0 = BITSEL(ir, 15);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_memory_atomicops_cas_pair_decode4661064&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 126:
        {
          // Node 1301
          // Node 1302
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 9:
            {
              // Node 1303
              // Node 1304
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1305
                  // Node 1306
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_fp_simd_decode4660225;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4660225&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 62:
        {
          // Node 1307
          // Node 1308
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 9:
            {
              // Node 1309
              // Node 1310
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1311
                  // Node 1312
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_fp_simd_decode4662030;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_fp_simd_decode4662030&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 2:
            {
              // Node 1385
              // Node 1386
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1387
                  // Node 1388
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4660905&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 6:
            {
              // Node 1389
              // Node 1390
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1391
                  // Node 1392
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_fp_simd_decode4661715&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 124:
        {
          // Node 1431
          // Node 1432
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 0:
            {
              // Node 1433
              // Node 1434
              switch (UNSIGNED_BITS(ir, 13,12)) 
              {
              case 0:
                {
                  // Node 1435
                  // Node 1436
                  switch ((ir & BIT_LSB(10)) >> 10)
                  {
                  case 0:
                    {
                      // Node 1437
                      // Node 1438
                      opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).Q = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).sz = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).L = BITSEL(ir, 21);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).M = BITSEL(ir, 20);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).S = BITSEL(ir, 14);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).H = BITSEL(ir, 11);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_lower_decode4661926&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 125:
        {
          // Node 1439
          // Node 1440
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 1:
            {
              // Node 1441
              // Node 1442
              switch (UNSIGNED_BITS(ir, 13,12)) 
              {
              case 0:
                {
                  // Node 1443
                  // Node 1444
                  switch ((ir & BIT_LSB(10)) >> 10)
                  {
                  case 0:
                    {
                      // Node 1445
                      // Node 1446
                      opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).Q = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).sz = BITSEL(ir, 22);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).L = BITSEL(ir, 21);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).M = BITSEL(ir, 20);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).S = BITSEL(ir, 14);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).H = BITSEL(ir, 11);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_mul_norounding_i_upper_decode4662187&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 14:
        {
          // Node 1461
          // Node 1462
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 157:
            {
              // Node 1947
              // Node 1948
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4660997&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 221:
            {
              // Node 1957
              // Node 1958
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_int_simd_decode4663150;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663150&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663150&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663150&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663150&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663150&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 181:
            {
              // Node 1965
              // Node 1966
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_narrow_decode4661792;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_narrow_decode4661792&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_narrow_decode4661792&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_narrow_decode4661792&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_narrow_decode4661792&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_narrow_decode4661792&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 245:
            {
              // Node 1987
              // Node 1988
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_widen_decode4661107;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_widen_decode4661107&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_widen_decode4661107&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_widen_decode4661107&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_widen_decode4661107&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_widen_decode4661107&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1463
              // Node 1464
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 23:
                {
                  // Node 1465
                  // Node 1466
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp_decode4662516;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4662516&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 55:
                {
                  // Node 1685
                  // Node 1686
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_extended_simd_decode4660644&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 63:
                {
                  // Node 1729
                  // Node 1730
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_recps_simd_decode4662639;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_recps_simd_decode4662639&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 78:
        {
          // Node 1467
          // Node 1468
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 157:
            {
              // Node 1949
              // Node 1950
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_tieaway_simd_decode4662783&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 221:
            {
              // Node 1959
              // Node 1960
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_int_simd_decode4663155;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663155&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663155&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663155&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663155&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_int_simd_decode4663155&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 181:
            {
              // Node 1989
              // Node 1990
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_xtn_simd_decode4662345;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_simd_decode4662345&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_simd_decode4662345&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_simd_decode4662345&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_simd_decode4662345&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_xtn_simd_decode4662345&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1469
              // Node 1470
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 23:
                {
                  // Node 1471
                  // Node 1472
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp_decode4663119;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_decode4663119&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 63:
                {
                  // Node 1595
                  // Node 1596
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_div_decode4661518;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_div_decode4661518&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_div_decode4661518&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_div_decode4661518&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_div_decode4661518&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_div_decode4661518&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_div_decode4661518&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 55:
                {
                  // Node 1709
                  // Node 1710
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_product_decode4662540&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 28:
        {
          // Node 1529
          // Node 1530
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 2825:
            {
              // Node 1913
              // Node 1914
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660069&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2829:
            {
              // Node 1921
              // Node 1922
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662202&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 781:
            {
              // Node 1969
              // Node 1970
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_round_decode4660577;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660577&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660577&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660577&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660577&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660577&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 777:
            {
              // Node 1973
              // Node 1974
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_round_decode4662228;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662228&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662228&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662228&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662228&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662228&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1531
              // Node 1532
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 29:
                {
                  // Node 1533
                  // Node 1534
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660247&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 31:
                {
                  // Node 1641
                  // Node 1642
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662416&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 7:
                {
                  // Node 1651
                  // Node 1652
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660605&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 39:
                {
                  // Node 1695
                  // Node 1696
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4662153&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 92:
        {
          // Node 1535
          // Node 1536
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 2825:
            {
              // Node 1917
              // Node 1918
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661175&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2829:
            {
              // Node 1923
              // Node 1924
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4662601&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 781:
            {
              // Node 1967
              // Node 1968
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_round_decode4660314;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660314&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660314&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660314&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660314&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4660314&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 777:
            {
              // Node 1977
              // Node 1978
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_round_decode4663556;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663556&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663556&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663556&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663556&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663556&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 505:
            {
              // Node 2176
              // Node 2177
              opcode = aarch64_aarch64_vector_reduce_fpmax_simd_decode4662309;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4662309&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4662309&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4662309&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4662309&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4662309&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 409:
            {
              // Node 2182
              // Node 2183
              opcode = aarch64_aarch64_vector_reduce_fpmaxnm_simd_decode4662935;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4662935&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4662935&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4662935&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4662935&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4662935&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1537
              // Node 1538
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 29:
                {
                  // Node 1539
                  // Node 1540
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660373&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 31:
                {
                  // Node 1541
                  // Node 1542
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4660965&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 7:
                {
                  // Node 1653
                  // Node 1654
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4661832&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 93:
        {
          // Node 1543
          // Node 1544
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 2825:
            {
              // Node 1919
              // Node 1920
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4661216&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2829:
            {
              // Node 1927
              // Node 1928
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663700&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 781:
            {
              // Node 1975
              // Node 1976
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_round_decode4662675;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662675&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662675&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662675&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662675&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4662675&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 505:
            {
              // Node 2174
              // Node 2175
              opcode = aarch64_aarch64_vector_reduce_fpmax_simd_decode4660185;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4660185&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4660185&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4660185&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4660185&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmax_simd_decode4660185&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 409:
            {
              // Node 2184
              // Node 2185
              opcode = aarch64_aarch64_vector_reduce_fpmaxnm_simd_decode4663384;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4663384&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4663384&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4663384&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4663384&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_fpmaxnm_simd_decode4663384&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1545
              // Node 1546
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 29:
                {
                  // Node 1547
                  // Node 1548
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4661373&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 31:
                {
                  // Node 1549
                  // Node 1550
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_fp_simd_decode4663407&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 7:
                {
                  // Node 1649
                  // Node 1650
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4660049&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 29:
        {
          // Node 1643
          // Node 1644
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 2825:
            {
              // Node 1915
              // Node 1916
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4660880&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2829:
            {
              // Node 1925
              // Node 1926
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_conv_float_bulk_simd_decode4663180&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 777:
            {
              // Node 1971
              // Node 1972
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_round_decode4661283;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4661283&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4661283&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4661283&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4661283&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4661283&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 781:
            {
              // Node 1979
              // Node 1980
              opcode = aarch64_aarch64_vector_arithmetic_unary_float_round_decode4663586;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663586&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663586&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663586&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663586&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_decode4663586&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1645
              // Node 1646
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 31:
                {
                  // Node 1647
                  // Node 1648
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_1985_decode4662545&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 7:
                {
                  // Node 1655
                  // Node 1656
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_fp_2008_decode4663521&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 39:
                {
                  // Node 1693
                  // Node 1694
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_fused_decode4660324&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 46:
        {
          // Node 1737
          // Node 1738
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 1549:
            {
              // Node 1815
              // Node 1816
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4661435&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1545:
            {
              // Node 1819
              // Node 1820
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662355&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 185:
            {
              // Node 1829
              // Node 1830
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_lessthan_simd_decode4663781&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 249:
            {
              // Node 1877
              // Node 1878
              opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_float_decode4663755;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4663755&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4663755&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4663755&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4663755&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4663755&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 221:
            {
              // Node 2071
              // Node 2072
              opcode = aarch64_aarch64_vector_arithmetic_unary_special_recip_float_simd_decode4663020;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_simd_decode4663020&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_simd_decode4663020&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_simd_decode4663020&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_simd_decode4663020&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_float_simd_decode4663020&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 157:
            {
              // Node 2079
              // Node 2080
              opcode = aarch64_aarch64_vector_arithmetic_unary_special_recip_int_decode4661797;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_int_decode4661797&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_int_decode4661797&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_int_decode4661797&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_int_decode4661797&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_recip_int_decode4661797&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1739
              // Node 1740
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 63:
                {
                  // Node 1741
                  // Node 1742
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_rsqrts_simd_decode4663794&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 23:
                {
                  // Node 1775
                  // Node 1776
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4660174&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 110:
        {
          // Node 1777
          // Node 1778
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 1545:
            {
              // Node 1813
              // Node 1814
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4660695&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 1549:
            {
              // Node 1817
              // Node 1818
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_float_bulk_simd_decode4662131&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 249:
            {
              // Node 1875
              // Node 1876
              opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_float_decode4659939;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4659939&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4659939&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4659939&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4659939&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_float_decode4659939&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 253:
            {
              // Node 2081
              // Node 2082
              opcode = aarch64_aarch64_vector_arithmetic_unary_special_sqrt_decode4661827;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrt_decode4661827&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrt_decode4661827&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrt_decode4661827&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrt_decode4661827&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrt_decode4661827&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 221:
            {
              // Node 2083
              // Node 2084
              opcode = aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_float_simd_decode4661677&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 157:
            {
              // Node 2091
              // Node 2092
              opcode = aarch64_aarch64_vector_arithmetic_unary_special_sqrtest_int_decode4660547;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_int_decode4660547&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_int_decode4660547&)*this).sz = BITSEL(ir, 22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_int_decode4660547&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_int_decode4660547&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_special_sqrtest_int_decode4660547&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1779
              // Node 1780
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 23:
                {
                  // Node 1781
                  // Node 1782
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_sub_fp_simd_decode4661489&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 94:
        {
          // Node 2198
          // Node 2199
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 63:
            {
              // Node 2200
              // Node 2201
              opcode = aarch64_aarch64_vector_shift_conv_float_simd_decode4661860;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4661860&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4661860&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4661860&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4661860&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4661860&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4661860&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 15:
            {
              // Node 2212
              // Node 2213
              opcode = aarch64_aarch64_vector_shift_conv_int_simd_decode4661170;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4661170&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4661170&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4661170&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4661170&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4661170&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4661170&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 21:
            {
              // Node 2224
              // Node 2225
              opcode = aarch64_aarch64_vector_shift_leftinsert_simd_decode4662558;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_simd_decode4662558&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_simd_decode4662558&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_simd_decode4662558&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_simd_decode4662558&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_simd_decode4662558&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftinsert_simd_decode4662558&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 11:
            {
              // Node 2228
              // Node 2229
              opcode = aarch64_aarch64_vector_shift_leftlong_decode4660621;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4660621&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4660621&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4660621&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4660621&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4660621&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4660621&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 54:
            {
              // Node 2232
              // Node 2233
              opcode = aarch64_aarch64_vector_shift_leftsat_simd_decode4660748;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4660748&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4660748&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4660748&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4660748&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4660748&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4660748&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 50:
            {
              // Node 2236
              // Node 2237
              opcode = aarch64_aarch64_vector_shift_leftsat_simd_decode4661882;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661882&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661882&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661882&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661882&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661882&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661882&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 6:
            {
              // Node 2246
              // Node 2247
              opcode = aarch64_aarch64_vector_shift_right_simd_decode4660888;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660888&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660888&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660888&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660888&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660888&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660888&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 10:
            {
              // Node 2250
              // Node 2251
              opcode = aarch64_aarch64_vector_shift_right_simd_decode4661574;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661574&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661574&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661574&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661574&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661574&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661574&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 2254
              // Node 2255
              opcode = aarch64_aarch64_vector_shift_right_simd_decode4662298;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662298&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662298&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662298&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662298&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662298&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662298&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 14:
            {
              // Node 2258
              // Node 2259
              opcode = aarch64_aarch64_vector_shift_right_simd_decode4663531;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663531&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663531&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663531&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663531&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663531&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663531&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 5:
            {
              // Node 2276
              // Node 2277
              opcode = aarch64_aarch64_vector_shift_rightinsert_simd_decode4660867;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_simd_decode4660867&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_simd_decode4660867&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_simd_decode4660867&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_simd_decode4660867&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_simd_decode4660867&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightinsert_simd_decode4660867&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 7:
            {
              // Node 2284
              // Node 2285
              opcode = aarch64_aarch64_vector_shift_rightnarrow_nonuniform_simd_decode4662608;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_nonuniform_simd_decode4662608&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 39:
            {
              // Node 2288
              // Node 2289
              opcode = aarch64_aarch64_vector_shift_rightnarrow_uniform_simd_decode4660319;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4660319&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4660319&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4660319&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4660319&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4660319&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4660319&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 37:
            {
              // Node 2294
              // Node 2295
              opcode = aarch64_aarch64_vector_shift_rightnarrow_uniform_simd_decode4663802;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4663802&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4663802&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4663802&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4663802&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4663802&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4663802&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 30:
        {
          // Node 2202
          // Node 2203
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 63:
            {
              // Node 2204
              // Node 2205
              opcode = aarch64_aarch64_vector_shift_conv_float_simd_decode4663763;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4663763&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4663763&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4663763&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4663763&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4663763&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_float_simd_decode4663763&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 15:
            {
              // Node 2214
              // Node 2215
              opcode = aarch64_aarch64_vector_shift_conv_int_simd_decode4662247;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4662247&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4662247&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4662247&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4662247&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4662247&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_conv_int_simd_decode4662247&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 21:
            {
              // Node 2220
              // Node 2221
              opcode = aarch64_aarch64_vector_shift_left_simd_decode4660772;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_left_simd_decode4660772&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_simd_decode4660772&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_simd_decode4660772&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_simd_decode4660772&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_simd_decode4660772&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_left_simd_decode4660772&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 11:
            {
              // Node 2230
              // Node 2231
              opcode = aarch64_aarch64_vector_shift_leftlong_decode4661102;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4661102&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4661102&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4661102&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4661102&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4661102&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftlong_decode4661102&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 54:
            {
              // Node 2234
              // Node 2235
              opcode = aarch64_aarch64_vector_shift_leftsat_simd_decode4661587;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661587&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661587&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661587&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661587&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661587&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_leftsat_simd_decode4661587&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 14:
            {
              // Node 2244
              // Node 2245
              opcode = aarch64_aarch64_vector_shift_right_simd_decode4660863;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660863&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660863&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660863&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660863&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660863&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4660863&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 6:
            {
              // Node 2248
              // Node 2249
              opcode = aarch64_aarch64_vector_shift_right_simd_decode4661496;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661496&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661496&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661496&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661496&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661496&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4661496&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 10:
            {
              // Node 2252
              // Node 2253
              opcode = aarch64_aarch64_vector_shift_right_simd_decode4662100;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662100&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662100&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662100&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662100&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662100&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4662100&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 2256
              // Node 2257
              opcode = aarch64_aarch64_vector_shift_right_simd_decode4663494;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663494&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663494&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663494&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663494&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663494&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_right_simd_decode4663494&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 7:
            {
              // Node 2280
              // Node 2281
              opcode = aarch64_aarch64_vector_shift_rightnarrow_logical_decode4661246;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4661246&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4661246&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4661246&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4661246&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4661246&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4661246&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 5:
            {
              // Node 2282
              // Node 2283
              opcode = aarch64_aarch64_vector_shift_rightnarrow_logical_decode4663010;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4663010&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4663010&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4663010&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4663010&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4663010&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_logical_decode4663010&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 39:
            {
              // Node 2290
              // Node 2291
              opcode = aarch64_aarch64_vector_shift_rightnarrow_uniform_simd_decode4662217;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662217&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662217&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662217&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662217&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662217&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662217&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 37:
            {
              // Node 2292
              // Node 2293
              opcode = aarch64_aarch64_vector_shift_rightnarrow_uniform_simd_decode4662628;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662628&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662628&)*this).immh = UNSIGNED_BITS(ir, 22,19);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662628&)*this).immb = UNSIGNED_BITS(ir, 18,16);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662628&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662628&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_shift_rightnarrow_uniform_simd_decode4662628&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 29,24)) 
      {
      case 48:
        {
          // Node 702
          // Node 703
          opcode = aarch64_aarch64_memory_literal_general_decode4660756;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4660756&)*this).opc = BITSEL(ir, 30);
          ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4660756&)*this).imm19 = UNSIGNED_BITS(ir, 23,5);
          ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4660756&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_literal_general_decode4660756&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 14:
        {
          // Node 1195
          // Node 1196
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 53:
            {
              // Node 1793
              // Node 1794
              opcode = aarch64_aarch64_vector_arithmetic_unary_add_pairwise_decode4660090;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4660090&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4660090&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4660090&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4660090&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4660090&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 37:
            {
              // Node 1797
              // Node 1798
              opcode = aarch64_aarch64_vector_arithmetic_unary_add_pairwise_decode4661475;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661475&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661475&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661475&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661475&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661475&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 225:
            {
              // Node 1803
              // Node 1804
              opcode = aarch64_aarch64_vector_arithmetic_unary_add_saturating_simd_decode4662464;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4662464&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4662464&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4662464&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4662464&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4662464&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 145:
            {
              // Node 1811
              // Node 1812
              opcode = aarch64_aarch64_vector_arithmetic_unary_clsz_decode4663218;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4663218&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4663218&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4663218&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4663218&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4663218&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 525:
            {
              // Node 1853
              // Node 1854
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4660123&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 521:
            {
              // Node 1857
              // Node 1858
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663069&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 169:
            {
              // Node 1869
              // Node 1870
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_lessthan_simd_decode4660538&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 209:
            {
              // Node 1873
              // Node 1874
              opcode = aarch64_aarch64_vector_arithmetic_unary_cnt_decode4663731;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cnt_decode4663731&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cnt_decode4663731&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cnt_decode4663731&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cnt_decode4663731&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cnt_decode4663731&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 233:
            {
              // Node 1883
              // Node 1884
              opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_int_simd_decode4659980;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4659980&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4659980&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4659980&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4659980&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4659980&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 241:
            {
              // Node 1891
              // Node 1892
              opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_sat_simd_decode4661871;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4661871&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4661871&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4661871&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4661871&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4661871&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 165:
            {
              // Node 1899
              // Node 1900
              opcode = aarch64_aarch64_vector_arithmetic_unary_extract_nosat_decode4660843;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_nosat_decode4660843&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_nosat_decode4660843&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_nosat_decode4660843&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_nosat_decode4660843&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_nosat_decode4660843&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 149:
            {
              // Node 1903
              // Node 1904
              opcode = aarch64_aarch64_vector_arithmetic_unary_extract_sat_simd_decode4662161;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4662161&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4662161&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4662161&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4662161&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4662161&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 13:
            {
              // Node 2059
              // Node 2060
              opcode = aarch64_aarch64_vector_arithmetic_unary_rev_decode4660516;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660516&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660516&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660516&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660516&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660516&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 9:
            {
              // Node 2063
              // Node 2064
              opcode = aarch64_aarch64_vector_arithmetic_unary_rev_decode4663526;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4663526&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4663526&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4663526&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4663526&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4663526&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 493:
            {
              // Node 2146
              // Node 2147
              opcode = aarch64_aarch64_vector_reduce_add_simd_decode4662836;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_add_simd_decode4662836&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_add_simd_decode4662836&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_add_simd_decode4662836&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_add_simd_decode4662836&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_add_simd_decode4662836&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 481:
            {
              // Node 2152
              // Node 2153
              opcode = aarch64_aarch64_vector_reduce_addlong_decode4663817;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4663817&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4663817&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4663817&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4663817&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4663817&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 405:
            {
              // Node 2190
              // Node 2191
              opcode = aarch64_aarch64_vector_reduce_intmax_decode4660141;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660141&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660141&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660141&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660141&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660141&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 469:
            {
              // Node 2194
              // Node 2195
              opcode = aarch64_aarch64_vector_reduce_intmax_decode4661618;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4661618&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4661618&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4661618&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4661618&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4661618&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1197
              // Node 1198
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 0:
                {
                  // Node 1199
                  // Node 1200
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_long_decode4660687;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4660687&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 8:
                {
                  // Node 1207
                  // Node 1208
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_long_decode4661481;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661481&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 40:
                {
                  // Node 1213
                  // Node 1214
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660414&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 32:
                {
                  // Node 1215
                  // Node 1216
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4661984&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 1:
                {
                  // Node 1219
                  // Node 1220
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_wide_decode4661455;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4661455&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 9:
                {
                  // Node 1223
                  // Node 1224
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_wide_decode4662076;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662076&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 33:
                {
                  // Node 1227
                  // Node 1228
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_diff_decode4659934;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4659934&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4659934&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4659934&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4659934&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4659934&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4659934&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 41:
                {
                  // Node 1229
                  // Node 1230
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_diff_decode4660959;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4660959&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4660959&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4660959&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4660959&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4660959&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4660959&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 24:
                {
                  // Node 1235
                  // Node 1236
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_accum_decode4660838;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4660838&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 16:
                {
                  // Node 1239
                  // Node 1240
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_accum_decode4662665;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4662665&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 17:
                {
                  // Node 1243
                  // Node 1244
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4662181&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 25:
                {
                  // Node 1245
                  // Node 1246
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_dmacc_simd_decode4663503&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 44:
                {
                  // Node 1255
                  // Node 1256
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_double_simd_decode4663253&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 28:
                {
                  // Node 1259
                  // Node 1260
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_poly_decode4662475;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_poly_decode4662475&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 48:
                {
                  // Node 1261
                  // Node 1262
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_product_decode4662951;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4662951&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 59:
                {
                  // Node 1483
                  // Node 1484
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_pair_decode4659901&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 3:
                {
                  // Node 1485
                  // Node 1486
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660367&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 35:
                {
                  // Node 1495
                  // Node 1496
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4662445&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 49:
                {
                  // Node 1573
                  // Node 1574
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4659808&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 51:
                {
                  // Node 1579
                  // Node 1580
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661846&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 57:
                {
                  // Node 1593
                  // Node 1594
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_diff_decode4663026;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4663026&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4663026&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4663026&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4663026&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4663026&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4663026&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 21:
                {
                  // Node 1657
                  // Node 1658
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659768&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 23:
                {
                  // Node 1663
                  // Node 1664
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4662223&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 27:
                {
                  // Node 1667
                  // Node 1668
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_single_decode4662724;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662724&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 19:
                {
                  // Node 1711
                  // Node 1712
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662404&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 46:
        {
          // Node 1201
          // Node 1202
          switch (UNSIGNED_BITS(ir, 21,10)) 
          {
          case 53:
            {
              // Node 1795
              // Node 1796
              opcode = aarch64_aarch64_vector_arithmetic_unary_add_pairwise_decode4661018;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661018&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661018&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661018&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661018&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4661018&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 37:
            {
              // Node 1799
              // Node 1800
              opcode = aarch64_aarch64_vector_arithmetic_unary_add_pairwise_decode4662243;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4662243&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4662243&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4662243&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4662243&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_pairwise_decode4662243&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 225:
            {
              // Node 1801
              // Node 1802
              opcode = aarch64_aarch64_vector_arithmetic_unary_add_saturating_simd_decode4660656;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4660656&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4660656&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4660656&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4660656&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_add_saturating_simd_decode4660656&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 145:
            {
              // Node 1809
              // Node 1810
              opcode = aarch64_aarch64_vector_arithmetic_unary_clsz_decode4662000;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4662000&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4662000&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4662000&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4662000&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_clsz_decode4662000&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 521:
            {
              // Node 1855
              // Node 1856
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4661120&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 525:
            {
              // Node 1859
              // Node 1860
              opcode = aarch64_aarch64_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_cmp_int_bulk_simd_decode4663081&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 233:
            {
              // Node 1885
              // Node 1886
              opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_int_simd_decode4660274;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4660274&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4660274&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4660274&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4660274&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_int_simd_decode4660274&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 241:
            {
              // Node 1893
              // Node 1894
              opcode = aarch64_aarch64_vector_arithmetic_unary_diffneg_sat_simd_decode4662592;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4662592&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4662592&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4662592&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4662592&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_diffneg_sat_simd_decode4662592&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 149:
            {
              // Node 1901
              // Node 1902
              opcode = aarch64_aarch64_vector_arithmetic_unary_extract_sat_simd_decode4660257;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4660257&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4660257&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4660257&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4660257&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sat_simd_decode4660257&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 165:
            {
              // Node 1909
              // Node 1910
              opcode = aarch64_aarch64_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_extract_sqxtun_simd_decode4662105&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 9:
            {
              // Node 2061
              // Node 2062
              opcode = aarch64_aarch64_vector_arithmetic_unary_rev_decode4660558;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660558&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660558&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660558&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660558&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_rev_decode4660558&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 229:
            {
              // Node 2065
              // Node 2066
              opcode = aarch64_aarch64_vector_arithmetic_unary_shift_decode4663581;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_shift_decode4663581&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_shift_decode4663581&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_shift_decode4663581&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_shift_decode4663581&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_shift_decode4663581&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 481:
            {
              // Node 2150
              // Node 2151
              opcode = aarch64_aarch64_vector_reduce_addlong_decode4662233;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4662233&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4662233&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4662233&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4662233&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_addlong_decode4662233&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 469:
            {
              // Node 2192
              // Node 2193
              opcode = aarch64_aarch64_vector_reduce_intmax_decode4660298;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660298&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660298&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660298&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660298&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4660298&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 405:
            {
              // Node 2196
              // Node 2197
              opcode = aarch64_aarch64_vector_reduce_intmax_decode4662918;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4662918&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4662918&)*this).size = UNSIGNED_BITS(ir, 23,22);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4662918&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4662918&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_reduce_intmax_decode4662918&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1203
              // Node 1204
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 0:
                {
                  // Node 1205
                  // Node 1206
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_long_decode4661384;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661384&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 8:
                {
                  // Node 1209
                  // Node 1210
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_long_decode4661538;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_long_decode4661538&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 40:
                {
                  // Node 1211
                  // Node 1212
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4660384&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 32:
                {
                  // Node 1217
                  // Node 1218
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_narrow_decode4662634&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 1:
                {
                  // Node 1221
                  // Node 1222
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_wide_decode4662052;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662052&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 9:
                {
                  // Node 1225
                  // Node 1226
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_addsub_wide_decode4662304;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_addsub_wide_decode4662304&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 33:
                {
                  // Node 1231
                  // Node 1232
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_diff_decode4661040;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661040&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661040&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661040&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661040&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661040&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661040&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 41:
                {
                  // Node 1233
                  // Node 1234
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_diff_decode4661866;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661866&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661866&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661866&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661866&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661866&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_diff_decode4661866&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 16:
                {
                  // Node 1237
                  // Node 1238
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_accum_decode4661157;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4661157&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 24:
                {
                  // Node 1241
                  // Node 1242
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_accum_decode4663433;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_accum_decode4663433&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 48:
                {
                  // Node 1263
                  // Node 1264
                  opcode = aarch64_aarch64_vector_arithmetic_binary_disparate_mul_product_decode4663511;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_disparate_mul_product_decode4663511&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 17:
                {
                  // Node 1473
                  // Node 1474
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_halving_rounding_decode4663516&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 3:
                {
                  // Node 1487
                  // Node 1488
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_wrapping_single_simd_decode4660848&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 35:
                {
                  // Node 1493
                  // Node 1494
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_bitwise_simd_decode4660723&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 51:
                {
                  // Node 1575
                  // Node 1576
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4660288&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 49:
                {
                  // Node 1577
                  // Node 1578
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_cmp_int_simd_decode4661721&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 57:
                {
                  // Node 1589
                  // Node 1590
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_diff_decode4660953;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660953&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660953&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660953&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660953&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660953&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4660953&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 59:
                {
                  // Node 1591
                  // Node 1592
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_diff_decode4662320;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4662320&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4662320&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4662320&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4662320&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4662320&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_diff_decode4662320&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 23:
                {
                  // Node 1659
                  // Node 1660
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4659779&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 21:
                {
                  // Node 1661
                  // Node 1662
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_pair_decode4661649&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 25:
                {
                  // Node 1665
                  // Node 1666
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_single_decode4662399;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4662399&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 27:
                {
                  // Node 1669
                  // Node 1670
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_maxmin_single_decode4663064;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_maxmin_single_decode4663064&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 19:
                {
                  // Node 1713
                  // Node 1714
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_accum_decode4662903&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 0:
            {
              // Node 1455
              // Node 1456
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 7:
                {
                  // Node 1715
                  // Node 1716
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4660147&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 5:
                {
                  // Node 1717
                  // Node 1718
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_int_doubling_accum_simd_decode4661508&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              switch (UNSIGNED_BITS(ir, 15,13)) 
              {
              case 7:
                {
                  // Node 1457
                  // Node 1458
                  switch (UNSIGNED_BITS(ir, 11,10)) 
                  {
                  case 2:
                    {
                      // Node 1459
                      // Node 1460
                      opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034&)*this).Q = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034&)*this).size = UNSIGNED_BITS(ir, 23,22);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034&)*this).rot = BITSEL(ir, 12);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_add_fp_complex_decode4661034&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              case 3:
                {
                  // Node 1681
                  // Node 1682
                  switch ((ir & BIT_LSB(10)) >> 10)
                  {
                  case 1:
                    {
                      // Node 1683
                      // Node 1684
                      opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101&)*this).Q = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101&)*this).size = UNSIGNED_BITS(ir, 23,22);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101&)*this).rot = UNSIGNED_BITS(ir, 12,11);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_complex_decode4660101&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 30:
        {
          // Node 1265
          // Node 1266
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 13:
            {
              // Node 1267
              // Node 1268
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1269
                  // Node 1270
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_double_simd_decode4661780;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_double_simd_decode4661780&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 6:
            {
              // Node 1325
              // Node 1326
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1327
                  // Node 1328
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_high_simd_decode4661532;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661532&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 7:
            {
              // Node 1329
              // Node 1330
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1331
                  // Node 1332
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_high_simd_decode4661804;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_high_simd_decode4661804&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 1:
            {
              // Node 1341
              // Node 1342
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1343
                  // Node 1344
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_int_decode4661468;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_int_decode4661468&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 5:
            {
              // Node 1351
              // Node 1352
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1353
                  // Node 1354
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_long_decode4663607;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4663607&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 3:
            {
              // Node 1361
              // Node 1362
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1363
                  // Node 1364
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_double_simd_decode4662470&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 62:
        {
          // Node 1345
          // Node 1346
          switch (UNSIGNED_BITS(ir, 15,12)) 
          {
          case 5:
            {
              // Node 1347
              // Node 1348
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1349
                  // Node 1350
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mul_long_decode4661161;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mul_long_decode4661161&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 13:
            {
              // Node 1401
              // Node 1402
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1403
                  // Node 1404
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4661502&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 15:
            {
              // Node 1405
              // Node 1406
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1407
                  // Node 1408
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_high_simd_decode4662370&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 0:
            {
              // Node 1419
              // Node 1420
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1421
                  // Node 1422
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_int_decode4661601;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4661601&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 4:
            {
              // Node 1423
              // Node 1424
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1425
                  // Node 1426
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_int_decode4662684;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_int_decode4662684&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 1:
            {
              // Node 1427
              // Node 1428
              switch ((ir & BIT_LSB(10)) >> 10)
              {
              case 0:
                {
                  // Node 1429
                  // Node 1430
                  opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_long_decode4662581;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).L = BITSEL(ir, 21);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).M = BITSEL(ir, 20);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).H = BITSEL(ir, 11);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_long_decode4662581&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          switch ((ir & BIT_LSB(15)) >> 15)
          {
          case 0:
            {
              // Node 1355
              // Node 1356
              switch ((ir & BIT_LSB(12)) >> 12)
              {
              case 1:
                {
                  // Node 1357
                  // Node 1358
                  switch ((ir & BIT_LSB(10)) >> 10)
                  {
                  case 0:
                    {
                      // Node 1359
                      // Node 1360
                      opcode = aarch64_aarch64_vector_arithmetic_binary_element_mulacc_complex_decode4662170;
                      length = 4;
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).Q = BITSEL(ir, 30);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).size = UNSIGNED_BITS(ir, 23,22);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).L = BITSEL(ir, 21);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).M = BITSEL(ir, 20);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).Rm = UNSIGNED_BITS(ir, 19,16);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).rot = UNSIGNED_BITS(ir, 14,13);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).H = BITSEL(ir, 11);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                      ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_element_mulacc_complex_decode4662170&)*this).decode_behaviour();
                      is_predicated = false;
                      return true;
                      break;
                    }
                  }
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 28:
        {
          // Node 1697
          // Node 1698
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1699
              // Node 1700
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 55:
                {
                  // Node 1701
                  // Node 1702
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004&)*this).S = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_lower_decode4661004&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          case 0:
            {
              // Node 2334
              // Node 2335
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 21:
                {
                  // Node 2336
                  // Node 2337
                  opcode = aarch64_aarch64_vector_transfer_vector_permute_transpose_decode4660681;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4660681&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4660681&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4660681&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4660681&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4660681&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4660681&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 5:
                {
                  // Node 2338
                  // Node 2339
                  opcode = aarch64_aarch64_vector_transfer_vector_permute_transpose_decode4661702;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4661702&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4661702&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4661702&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4661702&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4661702&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_transpose_decode4661702&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 6:
                {
                  // Node 2340
                  // Node 2341
                  opcode = aarch64_aarch64_vector_transfer_vector_permute_unzip_decode4660564;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4660564&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4660564&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4660564&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4660564&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4660564&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4660564&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 22:
                {
                  // Node 2342
                  // Node 2343
                  opcode = aarch64_aarch64_vector_transfer_vector_permute_unzip_decode4661752;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4661752&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4661752&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4661752&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4661752&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4661752&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_unzip_decode4661752&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 23:
                {
                  // Node 2344
                  // Node 2345
                  opcode = aarch64_aarch64_vector_transfer_vector_permute_zip_decode4660731;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4660731&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4660731&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4660731&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4660731&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4660731&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4660731&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              case 7:
                {
                  // Node 2346
                  // Node 2347
                  opcode = aarch64_aarch64_vector_transfer_vector_permute_zip_decode4661321;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4661321&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4661321&)*this).size = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4661321&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4661321&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4661321&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_transfer_vector_permute_zip_decode4661321&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      case 29:
        {
          // Node 1703
          // Node 1704
          switch ((ir & BIT_LSB(21)) >> 21)
          {
          case 1:
            {
              // Node 1705
              // Node 1706
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 51:
                {
                  // Node 1707
                  // Node 1708
                  opcode = aarch64_aarch64_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188&)*this).S = BITSEL(ir, 23);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_binary_uniform_mul_fp_mul_norounding_upper_decode4661188&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      // Node 170
      switch (UNSIGNED_BITS(ir, 28,19)) 
      {
      case 30:
        {
          // Node 2129
          // Node 2136
          switch (UNSIGNED_BITS(ir, 15,10)) 
          {
          case 61:
            {
              // Node 2137
              // Node 2138
              opcode = aarch64_aarch64_vector_logical_decode4662755;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).op = BITSEL(ir, 29);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).a = BITSEL(ir, 18);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).b = BITSEL(ir, 17);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).c = BITSEL(ir, 16);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).d = BITSEL(ir, 9);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).e = BITSEL(ir, 8);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).f = BITSEL(ir, 7);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).g = BITSEL(ir, 6);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).h = BITSEL(ir, 5);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4662755&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          // Node 2130
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 1:
            {
              // Node 2131
              // Node 2132
              opcode = aarch64_aarch64_vector_logical_decode4659791;
              length = 4;
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).Q = BITSEL(ir, 30);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).op = BITSEL(ir, 29);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).a = BITSEL(ir, 18);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).b = BITSEL(ir, 17);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).c = BITSEL(ir, 16);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).cmode = UNSIGNED_BITS(ir, 15,12);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).d = BITSEL(ir, 9);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).e = BITSEL(ir, 8);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).f = BITSEL(ir, 7);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).g = BITSEL(ir, 6);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).h = BITSEL(ir, 5);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_vector_logical_decode4659791&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 28,23)) 
      {
      case 14:
        {
          // Node 1981
          // Node 1982
          switch (UNSIGNED_BITS(ir, 21,13)) 
          {
          case 481:
            {
              // Node 1983
              // Node 1984
              switch (UNSIGNED_BITS(ir, 11,10)) 
              {
              case 1:
                {
                  // Node 1985
                  // Node 1986
                  opcode = aarch64_aarch64_vector_arithmetic_unary_float_round_frint_32_64_decode4661045;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045&)*this).Q = BITSEL(ir, 30);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045&)*this).U = BITSEL(ir, 29);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045&)*this).sz = BITSEL(ir, 22);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045&)*this).op = BITSEL(ir, 12);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_vector_arithmetic_unary_float_round_frint_32_64_decode4661045&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 28,24)) 
      {
      case 1:
        {
          // Node 171
          // Node 172
          opcode = aarch64_aarch64_integer_arithmetic_address_pcrel_decode4662813;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4662813&)*this).immlo = UNSIGNED_BITS(ir, 30,29);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4662813&)*this).immhi = UNSIGNED_BITS(ir, 23,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4662813&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_address_pcrel_decode4662813&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  // Node 1
  switch (UNSIGNED_BITS(ir, 30,15)) 
  {
  case 2944:
    {
      // Node 325
      // Node 326
      switch (UNSIGNED_BITS(ir, 13,10)) 
      {
      case 4:
        {
          // Node 327
          // Node 328
          switch (UNSIGNED_BITS(ir, 4,0)) 
          {
          case 11:
            {
              // Node 329
              opcode = aarch64_aarch64_integer_flags_setf_decode4662979;
              length = 4;
              ((aarch64_decode_aarch64_fmt_integer_flags_setf_decode4662979&)*this).sf = BITSEL(ir, 31);
              ((aarch64_decode_aarch64_fmt_integer_flags_setf_decode4662979&)*this).sz = BITSEL(ir, 14);
              ((aarch64_decode_aarch64_fmt_integer_flags_setf_decode4662979&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_integer_flags_setf_decode4662979&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 30,19)) 
  {
  case 2476:
    {
      // Node 2367
      switch (UNSIGNED_BITS(ir, 18,10)) 
      {
      case 17:
        {
          // Node 221
          // Node 222
          opcode = aarch64_aarch64_integer_arithmetic_cnt_decode4660660;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4660660&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4660660&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4660660&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4660660&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 16:
        {
          // Node 223
          // Node 224
          opcode = aarch64_aarch64_integer_arithmetic_cnt_decode4663297;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4663297&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4663297&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4663297&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_cnt_decode4663297&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 265
          // Node 266
          opcode = aarch64_aarch64_integer_arithmetic_rbit_decode4662535;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rbit_decode4662535&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rbit_decode4662535&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rbit_decode4662535&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rbit_decode4662535&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 271
          // Node 272
          opcode = aarch64_aarch64_integer_arithmetic_rev_decode4663238;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663238&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663238&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663238&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663238&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 18,11)) 
      {
      case 1:
        {
          // Node 269
          // Node 270
          opcode = aarch64_aarch64_integer_arithmetic_rev_decode4663076;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663076&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663076&)*this).opc = BITSEL(ir, 10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663076&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663076&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_rev_decode4663076&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 30,21)) 
  {
  case 267:
    {
      // Node 173
      // Node 174
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 0:
        {
          // Node 175
          // Node 176
          opcode = aarch64_aarch64_integer_arithmetic_addsub_carry_decode4659891;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4659891&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4659891&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4659891&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4659891&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4659891&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 523:
    {
      // Node 177
      // Node 178
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 0:
        {
          // Node 179
          // Node 180
          opcode = aarch64_aarch64_integer_arithmetic_addsub_carry_decode4660241;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660241&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660241&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660241&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660241&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660241&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 779:
    {
      // Node 181
      // Node 182
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 0:
        {
          // Node 183
          // Node 184
          opcode = aarch64_aarch64_integer_arithmetic_addsub_carry_decode4660357;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660357&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660357&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660357&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660357&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4660357&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 11:
    {
      // Node 185
      // Node 186
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 0:
        {
          // Node 187
          // Node 188
          opcode = aarch64_aarch64_integer_arithmetic_addsub_carry_decode4661190;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4661190&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4661190&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4661190&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4661190&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_carry_decode4661190&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 721:
    {
      // Node 189
      // Node 190
      opcode = aarch64_aarch64_integer_arithmetic_addsub_extendedreg_decode4659985;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985&)*this).imm3 = UNSIGNED_BITS(ir, 12,10);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4659985&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 209:
    {
      // Node 191
      // Node 192
      opcode = aarch64_aarch64_integer_arithmetic_addsub_extendedreg_decode4660040;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040&)*this).imm3 = UNSIGNED_BITS(ir, 12,10);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4660040&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 465:
    {
      // Node 193
      // Node 194
      opcode = aarch64_aarch64_integer_arithmetic_addsub_extendedreg_decode4661387;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387&)*this).imm3 = UNSIGNED_BITS(ir, 12,10);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4661387&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 977:
    {
      // Node 195
      // Node 196
      opcode = aarch64_aarch64_integer_arithmetic_addsub_extendedreg_decode4663497;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497&)*this).imm3 = UNSIGNED_BITS(ir, 12,10);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_extendedreg_decode4663497&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 107:
    {
      // Node 225
      // Node 226
      switch (UNSIGNED_BITS(ir, 15,10)) 
      {
      case 33:
        {
          // Node 227
          // Node 228
          opcode = aarch64_aarch64_integer_arithmetic_div_decode4661638;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4661638&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4661638&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4661638&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4661638&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4661638&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 32:
        {
          // Node 229
          // Node 230
          opcode = aarch64_aarch64_integer_arithmetic_div_decode4663717;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4663717&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4663717&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4663717&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4663717&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_div_decode4663717&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 16:
        {
          // Node 414
          // Node 415
          opcode = aarch64_aarch64_integer_shift_variable_decode4661093;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661093&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661093&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661093&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661093&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661093&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 19:
        {
          // Node 416
          // Node 417
          opcode = aarch64_aarch64_integer_shift_variable_decode4661771;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661771&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661771&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661771&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661771&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4661771&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 17:
        {
          // Node 418
          // Node 419
          opcode = aarch64_aarch64_integer_shift_variable_decode4663366;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663366&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663366&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663366&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663366&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663366&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 18:
        {
          // Node 420
          // Node 421
          opcode = aarch64_aarch64_integer_shift_variable_decode4663826;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663826&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663826&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663826&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663826&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_shift_variable_decode4663826&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 15,12)) 
      {
      case 4:
        {
          // Node 311
          // Node 312
          opcode = aarch64_aarch64_integer_crc_decode4660733;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4660733&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4660733&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4660733&)*this).sz = UNSIGNED_BITS(ir, 11,10);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4660733&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4660733&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4660733&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 5:
        {
          // Node 313
          // Node 314
          opcode = aarch64_aarch64_integer_crc_decode4662762;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4662762&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4662762&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4662762&)*this).sz = UNSIGNED_BITS(ir, 11,10);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4662762&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4662762&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_crc_decode4662762&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 216:
    {
      // Node 231
      // Node 232
      switch ((ir & BIT_LSB(15)) >> 15)
      {
      case 1:
        {
          // Node 233
          // Node 234
          opcode = aarch64_aarch64_integer_arithmetic_mul_uniform_addsub_decode4660051;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4660051&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4660051&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4660051&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4660051&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4660051&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4660051&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 235
          // Node 236
          opcode = aarch64_aarch64_integer_arithmetic_mul_uniform_addsub_decode4661006;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4661006&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4661006&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4661006&)*this).Ra = UNSIGNED_BITS(ir, 14,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4661006&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4661006&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_mul_uniform_addsub_decode4661006&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 331:
    {
      // Node 279
      // Node 280
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 2:
        {
          // Node 281
          // Node 282
          switch ((ir & BIT_LSB(4)) >> 4)
          {
          case 0:
            {
              // Node 283
              // Node 284
              opcode = aarch64_aarch64_integer_conditional_compare_immediate_decode4660012;
              length = 4;
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4660012&)*this).sf = BITSEL(ir, 31);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4660012&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4660012&)*this).cond = UNSIGNED_BITS(ir, 15,12);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4660012&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4660012&)*this).nzcv = UNSIGNED_BITS(ir, 3,0);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4660012&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 0:
        {
          // Node 295
          // Node 296
          switch ((ir & BIT_LSB(4)) >> 4)
          {
          case 0:
            {
              // Node 297
              // Node 298
              opcode = aarch64_aarch64_integer_conditional_compare_register_decode4662874;
              length = 4;
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4662874&)*this).sf = BITSEL(ir, 31);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4662874&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4662874&)*this).cond = UNSIGNED_BITS(ir, 15,12);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4662874&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4662874&)*this).nzcv = UNSIGNED_BITS(ir, 3,0);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4662874&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 843:
    {
      // Node 285
      // Node 286
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 2:
        {
          // Node 287
          // Node 288
          switch ((ir & BIT_LSB(4)) >> 4)
          {
          case 0:
            {
              // Node 289
              // Node 290
              opcode = aarch64_aarch64_integer_conditional_compare_immediate_decode4662419;
              length = 4;
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4662419&)*this).sf = BITSEL(ir, 31);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4662419&)*this).imm5 = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4662419&)*this).cond = UNSIGNED_BITS(ir, 15,12);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4662419&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4662419&)*this).nzcv = UNSIGNED_BITS(ir, 3,0);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_immediate_decode4662419&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 0:
        {
          // Node 291
          // Node 292
          switch ((ir & BIT_LSB(4)) >> 4)
          {
          case 0:
            {
              // Node 293
              // Node 294
              opcode = aarch64_aarch64_integer_conditional_compare_register_decode4660228;
              length = 4;
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4660228&)*this).sf = BITSEL(ir, 31);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4660228&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4660228&)*this).cond = UNSIGNED_BITS(ir, 15,12);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4660228&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4660228&)*this).nzcv = UNSIGNED_BITS(ir, 3,0);
              ((aarch64_decode_aarch64_fmt_integer_conditional_compare_register_decode4660228&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 43:
    {
      // Node 299
      // Node 300
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 1:
        {
          // Node 301
          // Node 302
          opcode = aarch64_aarch64_integer_conditional_select_decode4660236;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4660236&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4660236&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4660236&)*this).cond = UNSIGNED_BITS(ir, 15,12);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4660236&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4660236&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4660236&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 303
          // Node 304
          opcode = aarch64_aarch64_integer_conditional_select_decode4661582;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661582&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661582&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661582&)*this).cond = UNSIGNED_BITS(ir, 15,12);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661582&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661582&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661582&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 555:
    {
      // Node 305
      // Node 306
      switch (UNSIGNED_BITS(ir, 11,10)) 
      {
      case 0:
        {
          // Node 307
          // Node 308
          opcode = aarch64_aarch64_integer_conditional_select_decode4661680;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661680&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661680&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661680&)*this).cond = UNSIGNED_BITS(ir, 15,12);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661680&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661680&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4661680&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 309
          // Node 310
          opcode = aarch64_aarch64_integer_conditional_select_decode4662744;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4662744&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4662744&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4662744&)*this).cond = UNSIGNED_BITS(ir, 15,12);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4662744&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4662744&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_conditional_select_decode4662744&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 46:
    {
      // Node 319
      // Node 320
      switch (UNSIGNED_BITS(ir, 14,10)) 
      {
      case 16:
        {
          // Node 321
          // Node 322
          switch ((ir & BIT_LSB(4)) >> 4)
          {
          case 0:
            {
              // Node 323
              // Node 324
              opcode = aarch64_aarch64_integer_flags_rmif_decode4662797;
              length = 4;
              ((aarch64_decode_aarch64_fmt_integer_flags_rmif_decode4662797&)*this).sf = BITSEL(ir, 31);
              ((aarch64_decode_aarch64_fmt_integer_flags_rmif_decode4662797&)*this).imm6 = UNSIGNED_BITS(ir, 20,15);
              ((aarch64_decode_aarch64_fmt_integer_flags_rmif_decode4662797&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_integer_flags_rmif_decode4662797&)*this).mask = UNSIGNED_BITS(ir, 3,0);
              ((aarch64_decode_aarch64_fmt_integer_flags_rmif_decode4662797&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 30,22)) 
  {
  case 160:
    {
      // Node 757
      // Node 758
      opcode = aarch64_aarch64_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572&)*this).opc_part1 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4660572&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 161:
    {
      // Node 759
      // Node 760
      opcode = aarch64_aarch64_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826&)*this).opc_part1 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_noalloc_memory_pair_general_noalloc__decode4662826&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 164:
    {
      // Node 763
      // Node 764
      opcode = aarch64_aarch64_memory_pair_general_offset_memory_pair_general_postidx__decode4661225;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225&)*this).opc_part1 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4661225&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 165:
    {
      // Node 765
      // Node 766
      opcode = aarch64_aarch64_memory_pair_general_offset_memory_pair_general_postidx__decode4663715;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715&)*this).opc_part1 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_offset_memory_pair_general_postidx__decode4663715&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 168:
    {
      // Node 767
      // Node 768
      opcode = aarch64_aarch64_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481&)*this).opc_part1 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4660481&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 169:
    {
      // Node 769
      // Node 770
      opcode = aarch64_aarch64_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204&)*this).opc_part1 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_postidx_memory_pair_general_postidx__decode4661204&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 173:
    {
      // Node 773
      // Node 774
      opcode = aarch64_aarch64_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782&)*this).opc_part1 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4659782&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 172:
    {
      // Node 777
      // Node 778
      opcode = aarch64_aarch64_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621&)*this).opc_part1 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_general_preidx_memory_pair_general_postidx__decode4661621&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 30,23)) 
  {
  case 25:
    {
      // Node 273
      // Node 274
      opcode = aarch64_aarch64_integer_bitfield_decode4661491;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491&)*this).N = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491&)*this).immr = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491&)*this).imms = UNSIGNED_BITS(ir, 15,10);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4661491&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 153:
    {
      // Node 275
      // Node 276
      opcode = aarch64_aarch64_integer_bitfield_decode4662760;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760&)*this).N = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760&)*this).immr = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760&)*this).imms = UNSIGNED_BITS(ir, 15,10);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4662760&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 89:
    {
      // Node 277
      // Node 278
      opcode = aarch64_aarch64_integer_bitfield_decode4663093;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093&)*this).N = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093&)*this).immr = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093&)*this).imms = UNSIGNED_BITS(ir, 15,10);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_bitfield_decode4663093&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 57:
    {
      // Node 331
      // Node 332
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 0:
        {
          // Node 333
          // Node 334
          opcode = aarch64_aarch64_integer_insext_extract_immediate_decode4661192;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192&)*this).N = BITSEL(ir, 22);
          ((aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192&)*this).imms = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_insext_extract_immediate_decode4661192&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 233:
    {
      // Node 335
      // Node 336
      opcode = aarch64_aarch64_integer_insext_insert_movewide_decode4660386;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4660386&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4660386&)*this).hw = UNSIGNED_BITS(ir, 22,21);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4660386&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4660386&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4660386&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 105:
    {
      // Node 337
      // Node 338
      opcode = aarch64_aarch64_integer_insext_insert_movewide_decode4661817;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4661817&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4661817&)*this).hw = UNSIGNED_BITS(ir, 22,21);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4661817&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4661817&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4661817&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 41:
    {
      // Node 339
      // Node 340
      opcode = aarch64_aarch64_integer_insext_insert_movewide_decode4662070;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4662070&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4662070&)*this).hw = UNSIGNED_BITS(ir, 22,21);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4662070&)*this).imm16 = UNSIGNED_BITS(ir, 20,5);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4662070&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_insext_insert_movewide_decode4662070&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 137:
    {
      // Node 341
      // Node 342
      opcode = aarch64_aarch64_integer_logical_immediate_decode4660553;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553&)*this).N = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553&)*this).immr = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553&)*this).imms = UNSIGNED_BITS(ir, 15,10);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4660553&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 73:
    {
      // Node 343
      // Node 344
      opcode = aarch64_aarch64_integer_logical_immediate_decode4662254;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254&)*this).N = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254&)*this).immr = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254&)*this).imms = UNSIGNED_BITS(ir, 15,10);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662254&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 9:
    {
      // Node 345
      // Node 346
      opcode = aarch64_aarch64_integer_logical_immediate_decode4662411;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411&)*this).N = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411&)*this).immr = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411&)*this).imms = UNSIGNED_BITS(ir, 15,10);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662411&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 201:
    {
      // Node 347
      // Node 348
      opcode = aarch64_aarch64_integer_logical_immediate_decode4662603;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603&)*this).N = BITSEL(ir, 22);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603&)*this).immr = UNSIGNED_BITS(ir, 21,16);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603&)*this).imms = UNSIGNED_BITS(ir, 15,10);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_logical_immediate_decode4662603&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 30,24)) 
  {
  case 44:
    {
      // Node 2
      // Node 3
      opcode = aarch64_aarch64_branch_conditional_compare_decode4660422;
      length = 4;
      ((aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4660422&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4660422&)*this).imm19 = UNSIGNED_BITS(ir, 23,5);
      ((aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4660422&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4660422&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 45:
    {
      // Node 4
      // Node 5
      opcode = aarch64_aarch64_branch_conditional_compare_decode4663551;
      length = 4;
      ((aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4663551&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4663551&)*this).imm19 = UNSIGNED_BITS(ir, 23,5);
      ((aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4663551&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_branch_conditional_compare_decode4663551&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 109:
    {
      // Node 10
      // Node 11
      opcode = aarch64_aarch64_branch_conditional_test_decode4660675;
      length = 4;
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4660675&)*this).b5 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4660675&)*this).b40 = UNSIGNED_BITS(ir, 23,19);
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4660675&)*this).imm14 = UNSIGNED_BITS(ir, 18,5);
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4660675&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4660675&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 108:
    {
      // Node 12
      // Node 13
      opcode = aarch64_aarch64_branch_conditional_test_decode4661704;
      length = 4;
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4661704&)*this).b5 = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4661704&)*this).b40 = UNSIGNED_BITS(ir, 23,19);
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4661704&)*this).imm14 = UNSIGNED_BITS(ir, 18,5);
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4661704&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_branch_conditional_test_decode4661704&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 15:
    {
      // Node 114
      // Node 115
      switch (UNSIGNED_BITS(ir, 21,10)) 
      {
      case 3328:
        {
          // Node 128
          // Node 129
          opcode = aarch64_aarch64_float_convert_int_decode4660814;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660814&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660814&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660814&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660814&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660814&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2432:
        {
          // Node 130
          // Node 131
          opcode = aarch64_aarch64_float_convert_int_decode4660937;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660937&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660937&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660937&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660937&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4660937&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2816:
        {
          // Node 132
          // Node 133
          opcode = aarch64_aarch64_float_convert_int_decode4661166;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661166&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661166&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661166&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661166&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661166&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3072:
        {
          // Node 134
          // Node 135
          opcode = aarch64_aarch64_float_convert_int_decode4661592;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661592&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661592&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661592&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661592&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661592&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3584:
        {
          // Node 136
          // Node 137
          opcode = aarch64_aarch64_float_convert_int_decode4661815;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661815&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661815&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661815&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661815&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661815&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2560:
        {
          // Node 138
          // Node 139
          opcode = aarch64_aarch64_float_convert_int_decode4661967;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661967&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661967&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661967&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661967&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4661967&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2176:
        {
          // Node 146
          // Node 147
          opcode = aarch64_aarch64_float_convert_int_decode4662409;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662409&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662409&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662409&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662409&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662409&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 3840:
        {
          // Node 148
          // Node 149
          opcode = aarch64_aarch64_float_convert_int_decode4662431;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662431&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662431&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662431&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662431&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662431&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2368:
        {
          // Node 150
          // Node 151
          opcode = aarch64_aarch64_float_convert_int_decode4662930;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662930&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662930&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662930&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662930&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662930&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2048:
        {
          // Node 152
          // Node 153
          opcode = aarch64_aarch64_float_convert_int_decode4662989;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662989&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662989&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662989&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662989&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662989&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2304:
        {
          // Node 154
          // Node 155
          opcode = aarch64_aarch64_float_convert_int_decode4663005;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663005&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663005&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663005&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663005&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663005&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2112:
        {
          // Node 158
          // Node 159
          opcode = aarch64_aarch64_float_convert_int_decode4663836;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663836&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663836&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663836&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663836&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_int_decode4663836&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 21,16)) 
      {
      case 24:
        {
          // Node 116
          // Node 117
          opcode = aarch64_aarch64_float_convert_fix_decode4659868;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4659868&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4659868&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4659868&)*this).scale = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4659868&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4659868&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4659868&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 2:
        {
          // Node 118
          // Node 119
          opcode = aarch64_aarch64_float_convert_fix_decode4660589;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660589&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660589&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660589&)*this).scale = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660589&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660589&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660589&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 28:
        {
          // Node 120
          // Node 121
          opcode = aarch64_aarch64_float_convert_fix_decode4660609;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660609&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660609&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660609&)*this).scale = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660609&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660609&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4660609&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 6:
        {
          // Node 122
          // Node 123
          opcode = aarch64_aarch64_float_convert_fix_decode4663376;
          length = 4;
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4663376&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4663376&)*this).typ = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4663376&)*this).scale = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4663376&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4663376&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_float_convert_fix_decode4663376&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      switch (UNSIGNED_BITS(ir, 21,20)) 
      {
      case 2:
        {
          // Node 140
          // Node 141
          switch (UNSIGNED_BITS(ir, 18,17)) 
          {
          case 3:
            {
              // Node 142
              // Node 143
              switch (UNSIGNED_BITS(ir, 15,10)) 
              {
              case 0:
                {
                  // Node 144
                  // Node 145
                  opcode = aarch64_aarch64_float_convert_int_decode4662018;
                  length = 4;
                  ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662018&)*this).sf = BITSEL(ir, 31);
                  ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662018&)*this).typ = UNSIGNED_BITS(ir, 23,22);
                  ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662018&)*this).rmode = BITSEL(ir, 19);
                  ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662018&)*this).opcode = BITSEL(ir, 16);
                  ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662018&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
                  ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662018&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
                  ((aarch64_decode_aarch64_fmt_float_convert_int_decode4662018&)*this).decode_behaviour();
                  is_predicated = false;
                  return true;
                  break;
                }
              }
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 17:
    {
      // Node 197
      // Node 198
      opcode = aarch64_aarch64_integer_arithmetic_addsub_immediate_decode4659928;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4659928&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4659928&)*this).shift = UNSIGNED_BITS(ir, 23,22);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4659928&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4659928&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4659928&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4659928&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 113:
    {
      // Node 199
      // Node 200
      opcode = aarch64_aarch64_integer_arithmetic_addsub_immediate_decode4660472;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660472&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660472&)*this).shift = UNSIGNED_BITS(ir, 23,22);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660472&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660472&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660472&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660472&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 49:
    {
      // Node 201
      // Node 202
      opcode = aarch64_aarch64_integer_arithmetic_addsub_immediate_decode4660774;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660774&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660774&)*this).shift = UNSIGNED_BITS(ir, 23,22);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660774&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660774&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660774&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4660774&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 81:
    {
      // Node 203
      // Node 204
      opcode = aarch64_aarch64_integer_arithmetic_addsub_immediate_decode4661399;
      length = 4;
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4661399&)*this).sf = BITSEL(ir, 31);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4661399&)*this).shift = UNSIGNED_BITS(ir, 23,22);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4661399&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4661399&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4661399&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_immediate_decode4661399&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 90:
    {
      // Node 205
      // Node 206
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 0:
        {
          // Node 207
          // Node 208
          opcode = aarch64_aarch64_integer_arithmetic_addsub_shiftedreg_decode4659762;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4659762&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 122:
    {
      // Node 209
      // Node 210
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 0:
        {
          // Node 211
          // Node 212
          opcode = aarch64_aarch64_integer_arithmetic_addsub_shiftedreg_decode4660207;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4660207&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 26:
    {
      // Node 213
      // Node 214
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 0:
        {
          // Node 215
          // Node 216
          opcode = aarch64_aarch64_integer_arithmetic_addsub_shiftedreg_decode4662735;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4662735&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 58:
    {
      // Node 217
      // Node 218
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 0:
        {
          // Node 219
          // Node 220
          opcode = aarch64_aarch64_integer_arithmetic_addsub_shiftedreg_decode4663364;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_arithmetic_addsub_shiftedreg_decode4663364&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 10:
    {
      // Node 349
      // Node 350
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 351
          // Node 352
          opcode = aarch64_aarch64_integer_logical_shiftedreg_decode4660725;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660725&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 367
          // Node 368
          opcode = aarch64_aarch64_integer_logical_shiftedreg_decode4663421;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663421&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 106:
    {
      // Node 353
      // Node 354
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 355
          // Node 356
          opcode = aarch64_aarch64_integer_logical_shiftedreg_decode4660895;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660895&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 361
          // Node 362
          opcode = aarch64_aarch64_integer_logical_shiftedreg_decode4662645;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4662645&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 74:
    {
      // Node 357
      // Node 358
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 1:
        {
          // Node 359
          // Node 360
          opcode = aarch64_aarch64_integer_logical_shiftedreg_decode4660912;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4660912&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 0:
        {
          // Node 369
          // Node 370
          opcode = aarch64_aarch64_integer_logical_shiftedreg_decode4663481;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663481&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  case 42:
    {
      // Node 363
      // Node 364
      switch ((ir & BIT_LSB(21)) >> 21)
      {
      case 0:
        {
          // Node 365
          // Node 366
          opcode = aarch64_aarch64_integer_logical_shiftedreg_decode4663175;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663175&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 371
          // Node 372
          opcode = aarch64_aarch64_integer_logical_shiftedreg_decode4663672;
          length = 4;
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672&)*this).sf = BITSEL(ir, 31);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672&)*this).shift = UNSIGNED_BITS(ir, 23,22);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672&)*this).imm6 = UNSIGNED_BITS(ir, 15,10);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672&)*this).Rd = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_integer_logical_shiftedreg_decode4663672&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  // Node 706
  switch (UNSIGNED_BITS(ir, 29,22)) 
  {
  case 177:
    {
      // Node 779
      // Node 780
      opcode = aarch64_aarch64_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4661915&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 176:
    {
      // Node 781
      // Node 782
      opcode = aarch64_aarch64_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_noalloc_memory_pair_simdfp_noalloc__decode4663706&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 180:
    {
      // Node 783
      // Node 784
      opcode = aarch64_aarch64_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4659951&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 181:
    {
      // Node 785
      // Node 786
      opcode = aarch64_aarch64_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_offset_memory_pair_simdfp_postidx__decode4660580&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 184:
    {
      // Node 787
      // Node 788
      opcode = aarch64_aarch64_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4661613&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 185:
    {
      // Node 789
      // Node 790
      opcode = aarch64_aarch64_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_postidx_memory_pair_simdfp_postidx__decode4662267&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 188:
    {
      // Node 791
      // Node 792
      opcode = aarch64_aarch64_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4660639&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 189:
    {
      // Node 793
      // Node 794
      opcode = aarch64_aarch64_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559&)*this).imm7 = UNSIGNED_BITS(ir, 21,15);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559&)*this).Rt2 = UNSIGNED_BITS(ir, 14,10);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_pair_simdfp_preidx_memory_pair_simdfp_postidx__decode4663559&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  }
  switch (UNSIGNED_BITS(ir, 29,24)) 
  {
  case 52:
    {
      // Node 707
      // Node 708
      opcode = aarch64_aarch64_memory_literal_simdfp_decode4661110;
      length = 4;
      ((aarch64_decode_aarch64_fmt_memory_literal_simdfp_decode4661110&)*this).opc = UNSIGNED_BITS(ir, 31,30);
      ((aarch64_decode_aarch64_fmt_memory_literal_simdfp_decode4661110&)*this).imm19 = UNSIGNED_BITS(ir, 23,5);
      ((aarch64_decode_aarch64_fmt_memory_literal_simdfp_decode4661110&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
      ((aarch64_decode_aarch64_fmt_memory_literal_simdfp_decode4661110&)*this).decode_behaviour();
      is_predicated = false;
      return true;
      break;
    }
  case 60:
    {
      // Node 1008
      // Node 1009
      switch (UNSIGNED_BITS(ir, 22,21)) 
      {
      case 2:
        {
          // Node 1010
          // Node 1011
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 0:
            {
              // Node 1012
              // Node 1013
              opcode = aarch64_aarch64_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252&)*this).size = UNSIGNED_BITS(ir, 31,30);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252&)*this).opc_part1 = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4660252&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 1020
              // Node 1021
              opcode = aarch64_aarch64_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940&)*this).size = UNSIGNED_BITS(ir, 31,30);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940&)*this).opc_part1 = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4662940&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3:
            {
              // Node 1022
              // Node 1023
              opcode = aarch64_aarch64_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613&)*this).size = UNSIGNED_BITS(ir, 31,30);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613&)*this).opc_part1 = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4662613&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 0:
        {
          // Node 1014
          // Node 1015
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 0:
            {
              // Node 1016
              // Node 1017
              opcode = aarch64_aarch64_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207&)*this).size = UNSIGNED_BITS(ir, 31,30);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207&)*this).opc_part1 = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_offset_normal_memory_single_simdfp_immediate_signed_offset_normal__decode4662207&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 2:
            {
              // Node 1018
              // Node 1019
              opcode = aarch64_aarch64_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940&)*this).size = UNSIGNED_BITS(ir, 31,30);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940&)*this).opc_part1 = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_postidx_memory_single_simdfp_immediate_signed_postidx__decode4661940&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          case 3:
            {
              // Node 1024
              // Node 1025
              opcode = aarch64_aarch64_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145&)*this).size = UNSIGNED_BITS(ir, 31,30);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145&)*this).opc_part1 = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145&)*this).imm9 = UNSIGNED_BITS(ir, 20,12);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_signed_preidx_memory_single_simdfp_immediate_signed_postidx__decode4663145&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 1:
        {
          // Node 1032
          // Node 1033
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 1:
            {
              // Node 1034
              // Node 1035
              opcode = aarch64_aarch64_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446&)*this).size = UNSIGNED_BITS(ir, 31,30);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446&)*this).opc_part1 = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4660446&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      case 3:
        {
          // Node 1036
          // Node 1037
          switch (UNSIGNED_BITS(ir, 11,10)) 
          {
          case 1:
            {
              // Node 1038
              // Node 1039
              opcode = aarch64_aarch64_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887;
              length = 4;
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887&)*this).size = UNSIGNED_BITS(ir, 31,30);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887&)*this).opc_part1 = BITSEL(ir, 23);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887&)*this).Rm = UNSIGNED_BITS(ir, 20,16);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887&)*this).option_name = UNSIGNED_BITS(ir, 15,13);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887&)*this).S = BITSEL(ir, 12);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
              ((aarch64_decode_aarch64_fmt_memory_single_simdfp_register_memory_single_simdfp_register__decode4661887&)*this).decode_behaviour();
              is_predicated = false;
              return true;
              break;
            }
          }
          break;
        }
      }
      break;
    }
  case 62:
    {
      // Node 1026
      // Node 1027
      switch ((ir & BIT_LSB(22)) >> 22)
      {
      case 0:
        {
          // Node 1028
          // Node 1029
          opcode = aarch64_aarch64_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315&)*this).size = UNSIGNED_BITS(ir, 31,30);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315&)*this).opc_part1 = BITSEL(ir, 23);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4661315&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      case 1:
        {
          // Node 1030
          // Node 1031
          opcode = aarch64_aarch64_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703;
          length = 4;
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703&)*this).size = UNSIGNED_BITS(ir, 31,30);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703&)*this).opc_part1 = BITSEL(ir, 23);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703&)*this).imm12 = UNSIGNED_BITS(ir, 21,10);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703&)*this).Rn = UNSIGNED_BITS(ir, 9,5);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703&)*this).Rt = UNSIGNED_BITS(ir, 4,0);
          ((aarch64_decode_aarch64_fmt_memory_single_simdfp_immediate_unsigned_memory_single_simdfp_immediate_signed_postidx__decode4663703&)*this).decode_behaviour();
          is_predicated = false;
          return true;
          break;
        }
      }
      break;
    }
  }
  return false;
}
