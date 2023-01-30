#pragma once

#include <memory>
#include <aarch64-decode.h>

using namespace std;
using namespace captive::arch::aarch64;

unique_ptr<aarch64_decode> new_decoder()
{
    return unique_ptr<aarch64_decode>(new aarch64_decode());
}

uint64_t opcode(const aarch64_decode &decoder)
{
    return (uint64_t)decoder.opcode;
}
