#include "wrapper.h"

#include <arm64-decode.h>
#include <arm64-disasm.h>
#include <new>

using namespace std;
using namespace captive::arch::arm64;

arm64_decode *new_decoder(void *buf)
{
    return new (buf) arm64_decode();
}

arm64_disasm *new_disassembler(void *buf)
{
    return new (buf) arm64_disasm();
}
