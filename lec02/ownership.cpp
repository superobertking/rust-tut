#include <cstdio>
#include <cstdlib>
#include <iostream>

struct String {
    // In C, we have to keep in mind of all the alias manually, e.g. we need to
    // free the return value of readline(3) and realpath(3), but not readdir(3).
    // In C++, use std::unique_ptr, std::shared_ptr instead of raw pointer,
    // but there are still alias violation problems.
    char *ptr;
    size_t len;
    size_t cap;

    String(const char *s) {
        cap = len = strlen(s) + 1;
        ptr = new char[cap]; // malloc(cap)
        memcpy(ptr, s, len);
        ptr[len] = '\0';
    }
    ~String() { delete[] ptr; }

    String(const String &s) {
        cap = s.cap;
        len = s.len;
        ptr = new char[cap];
        memcpy(ptr, s.ptr, len);
        ptr[len] = '\0';
    }
    String(String &&s) {
        cap = s.cap;
        len = s.len;
        ptr = s.ptr;
        s.cap = s.len = 0;
        s.ptr = nullptr;
    }
};

int main() {
    String h("hello");
    String h2 = h; // then this will be a copy construct
    std::cout << h.ptr << std::endl;
    printf("h %p\n", h.ptr);
    std::cout << h2.ptr << std::endl;
    printf("h2 %p\n", h2.ptr);
    String h3(std::move(h));
    printf("%p\n", h.ptr); // this will be a move construct
    // To emulate move semantics in pre-C++11 specifications, you need to
    // manually swap an empty value and the original value.
    std::cout << h3.ptr << std::endl;
    printf("h3 %p\n", h3.ptr);
    // ~h2
    // ~h
}
