#include <emscripten.h>
#include <string.h>

// in JS: [..."Wasabi is awesome"].map(c => (c.charCodeAt() ^ 0x42) - 0x02).join(", ")
char obfuscated_password[] = { 19, 33, 47, 33, 30, 41, 96, 41, 47, 96, 33, 51, 37, 47, 43, 45, 37 };

EMSCRIPTEN_KEEPALIVE
int check(char input[]) {
    for (int i=0; obfuscated_password[i] != 0; i++) {
        obfuscated_password[i] += 0x02;
        obfuscated_password[i] ^= 0x42;
    }
    return strcmp(input, obfuscated_password) == 0;
}
