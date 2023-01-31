#pragma once

#include <memory>
#include <arm64-decode.h>
#include <arm64-disasm.h>

using namespace std;
using namespace captive::arch::arm64;

unique_ptr<arm64_decode> new_decoder()
{
    return unique_ptr<arm64_decode>(new arm64_decode());
}

int32_t opcode(const arm64_decode &decoder)
{
    return (int32_t)decoder.opcode;
}

unique_ptr<arm64_disasm> new_disassembler()
{
    return unique_ptr<arm64_disasm>(new arm64_disasm());
}
