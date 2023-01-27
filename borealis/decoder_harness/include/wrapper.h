#pragma once

#include <memory>
#include <aarch64-decode.h>

std::unique_ptr<captive::arch::aarch64::aarch64_decode> new_decoder()
{
    return std::unique_ptr<captive::arch::aarch64::aarch64_decode>(new captive::arch::aarch64::aarch64_decode());
}
