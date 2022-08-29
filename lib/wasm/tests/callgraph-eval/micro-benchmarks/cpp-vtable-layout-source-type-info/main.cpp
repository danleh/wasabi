// Do not use stdio because it pulls a lot of code into the module. In particular allocators and formatting stuff.
// #include<cstdio>

class A {
  public:
    virtual int method() {
        // puts("A\n");
        return 23;
    }
};

class B : public A {
  public:
    virtual int method() {
        // puts("B\n");
        return 42;
    }
};

int __attribute__((noinline)) module1(bool condition) {
    // Both objects are allocated on the stack, the base pointer is in $l2 (for my compiler version).
    // the ctors of both is trivial (no fields), so the only thing being "allocated" is the vtable pointer.
    // In my case:
    A a; // at $l2+8, vtable at 1032
    B b; // at $l2, vtable at 1064

    A* obj;
    // You can see that depending on $p0, a select chooses one of the two pointers
    if (condition) {
        obj = &a;
    } else {
        obj = &b;
    }

    return obj->method();
}

// Second, unrelated class hierarchy:
// A smart analysis with access to the class hierarchies could figure out that C::method and D::method 
// can never be called from the call site in module1 (i.e., for A::method or B::method) and vice versa.
class C {
  public:
    virtual int method() {
        return 1;
    }
};

class D : public C {
  public:
    virtual int method() {
        return 2;
    }
};

int __attribute__((noinline)) module2(bool condition) {
    C c;
    D d;

    C* obj;
    if (condition) {
        obj = &c;
    } else {
        obj = &d;
    }

    return obj->method();
}

int main(int argc, char** argv) {
    return module1(argc > 1) + module2(argc > 1);
}