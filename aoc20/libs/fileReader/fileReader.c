#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "fileReader.h"

void readInput(char* filename, int length, char** lines) {
    FILE *fptr;

    fptr = fopen(filename, "r");

    if (fptr == NULL) {
        perror("fopen");
        return;
    }
    char line[100];
    int i = 0;
    while(fgets(line, sizeof(line), fptr)) {
        lines[i] = malloc(strlen(line) + 1);
        if(lines[i] == NULL) {
            break;
        }
        strcpy(lines[i], line);
        i++;
    }
    fclose(fptr);
}