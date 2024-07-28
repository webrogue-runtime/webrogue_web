#include "emscripten.h"
#include <emscripten/wasmfs.h>
#include <stdint.h>

extern void rust_main();

void c_emscripten_sleep(uint32_t milliseconds) {
  emscripten_sleep(milliseconds);
}

int main(int argc, const char **argv) {
  wasmfs_create_memory_backend();
  rust_main();
}
