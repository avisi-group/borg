#include "wrapper.h"
#include <arm64-decode.h>
#include <arm64-disasm.h>

using namespace std;
using namespace captive::arch::arm64;

arm64_decode *new_decoder()
{
    return new arm64_decode();
}

// int32_t opcode(const arm64_decode &decoder)
// {
//     return (int32_t)decoder.opcode;
// }

arm64_disasm *new_disassembler()
{
    return new arm64_disasm();
}
