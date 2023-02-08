#include <arm64-decode.h>
#include <arm64-disasm.h>

using namespace std;
using namespace captive::arch::arm64;

arm64_decode *new_decoder(void *buf);

arm64_disasm *new_disassembler(void *buf);
