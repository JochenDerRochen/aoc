#include "fileReader.h"
#include <stdlib.h>
#include <stdio.h>
int main() {
    char** lines = malloc(100 * sizeof(char*));
    readInput("/home/bartusek/develop/personal/aoc/aoc20/day2/input.txt", 10, lines);
    for(int i = 0; i<4; i++)
    {
        printf("%s",lines[i]);
    }
    return 0;
}