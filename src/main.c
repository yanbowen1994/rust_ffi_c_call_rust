//
// Created by root on 7/5/19.
//
#include <stdint.h>
#include <stdio.h>

extern const char* status(char*);
extern int backint();

int main() {
    char* input = "123";
    const char* output = status(input);
    printf(output);
    printf("\n");
    int output1 = backint();
    printf("%d\n", output1);
    return 0;
}