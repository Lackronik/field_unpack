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
