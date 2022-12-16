out := test
obj := aarch64-decode.o test.o

cc-prefix := aarch64-linux-gnu-
cxx := g++
cc-cxx := $(cc-prefix)$(cxx)
cc-objcopy := $(cc-prefix)objcopy

ldflags :=
cxxflags := -g -Wall -O3 -I. -std=gnu++20

all: $(out) input.bin

clean: .FORCE
	rm -f $(obj) input.o input.bin

$(out): $(obj)
	$(cxx) -o $@ $(ldflags) $(obj)

%.o: %.cpp aarch64-decode.h decode.h
	$(cxx) -c -o $@ $(cxxflags) $<

input.o: input.S
	$(cc-cxx) -c -o $@ $<

input.bin: input.o
	$(cc-objcopy) --dump-section .text=$@ $<

.PHONY: .FORCE
