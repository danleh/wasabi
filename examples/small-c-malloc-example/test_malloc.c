#include <stdio.h>
#include <stdlib.h>

struct Person {
    int age;
    int weight;
};

int main(int argc, char const *argv[]) {
    struct Person *joe = malloc(sizeof(struct Person));
    joe->age = 27;
    joe->weight = 90;

    printf("Joe weights %d kg and is %d years old\n", joe->weight, joe->age);
    return 0;
}



