#include "wrapper.h"
#include <arm64-decode.h>
#include <arm64-disasm.h>

using namespace std;
using namespace captive::arch::arm64;

arm64_decode *new_decoder()
{
    return new arm64_decode();
}

arm64_disasm *new_disassembler()
{
    return new arm64_disasm();
}

void destruct_decoder(arm64_decode *decoder)
{
    delete decoder;
}

void destruct_disassembler(arm64_disasm *disassembler)
{
    delete disassembler;
}
