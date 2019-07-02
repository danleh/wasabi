#include <emscripten.h>
#include <string.h>

char password[] = "WebAssembly is awesome";

EMSCRIPTEN_KEEPALIVE
int check(char input[]) {
    return strcmp(input, password) == 0;
}
