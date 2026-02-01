#ifndef Bitboards_D_HPP
#define Bitboards_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct Bitboards {
      uint64_t w_k;
      uint64_t w_q;
      uint64_t w_r;
      uint64_t w_b;
      uint64_t w_n;
      uint64_t w_p;
      uint64_t b_k;
      uint64_t b_q;
      uint64_t b_r;
      uint64_t b_b;
      uint64_t b_n;
      uint64_t b_p;
    };
    
    typedef struct Bitboards_option {union { Bitboards ok; }; bool is_ok; } Bitboards_option;
} // namespace capi
} // namespace


struct Bitboards {
  uint64_t w_k;
  uint64_t w_q;
  uint64_t w_r;
  uint64_t w_b;
  uint64_t w_n;
  uint64_t w_p;
  uint64_t b_k;
  uint64_t b_q;
  uint64_t b_r;
  uint64_t b_b;
  uint64_t b_n;
  uint64_t b_p;

  inline diplomat::capi::Bitboards AsFFI() const;
  inline static Bitboards FromFFI(diplomat::capi::Bitboards c_struct);
};


#endif // Bitboards_D_HPP
