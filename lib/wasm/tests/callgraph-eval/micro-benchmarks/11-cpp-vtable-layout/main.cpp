#include<cstdio>

class A {
  public:
    virtual void method() {
        puts("A\n");
    }
};

class B : public A {
  public:
    virtual void method() {
        puts("B\n");
    }
};

int main(int argc, char** argv) {
    // Both objects are allocated on the stack, the base pointer is in $l2 (for my compiler version).
    // the ctors of both is trivial (no fields), so the only thing being "allocated" is the vtable pointer.
    // In my case:
    A a; // at $l2+8, vtable at 1032
    B b; // at $l2, vtable at 1064

    A* obj;
    // You can see that depending on $p0, a select chooses one of the two pointers
    if (argc > 1) {
        obj = &a;
    } else {
        obj = &b;
    }
    // The call indirect has a double indirect load from the object pointers.
    // So the vtable is in the beginning of the object.
    obj->method();
    return 0;
}