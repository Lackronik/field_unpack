#ifndef COMPLEX_EXAMPLE_H
#define COMPLEX_EXAMPLE_H

typedef unsigned long ulong;

enum Status {
    OK,
    ERROR,
    UNKNOWN
};

struct Address {
    char street[50];
    int number;
};

typedef struct {
    char name[50];
    int age;
    float height;
    struct Address address;
    enum Status status;
} Person;

union Data {
    int i;
    float f;
    char str[20];
    struct Address addr;
    Person person;
};

#endif // COMPLEX_EXAMPLE_H
