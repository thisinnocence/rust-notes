#include <stdint.h>
#include <iostream>

// 这个函数由 C++ 提供给 Rust 调用（Rust 侧会声明 extern "C"）。
extern "C" int32_t cpp_mul(int32_t a, int32_t b) { return a * b; }

// 这两个函数由 Rust 侧导出给 C++ 调用。
extern "C" int32_t rust_add(int32_t a, int32_t b);
extern "C" int32_t rust_mul_via_cpp(int32_t a, int32_t b);

int main() {
  const int32_t a = 7;
  const int32_t b = 35;

  const int32_t sum = rust_add(a, b);            // C++ -> Rust
  const int32_t mul = rust_mul_via_cpp(a, b);    // C++ -> Rust -> C++

  std::cout << "cpp -> rust: " << a << " + " << b << " = " << sum << '\n';
  std::cout << "cpp -> rust -> cpp: " << a << " * " << b << " = " << mul << '\n';
  return 0;
}
