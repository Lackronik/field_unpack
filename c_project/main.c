#include "main.h"

struct Node {
    int id;
    union {
        union Data data;
        int value;
    } info;
    struct Node* next;
};

typedef struct Node ListNode;

struct Complex {
    char description[100];
    ListNode list;
    enum Status current_status;
};

typedef struct {
    char make[20];
    char model[20];
    int year;
    union {
        int vin;
        char reg[10];
    } identifier;
    struct Complex details;
} Car;
