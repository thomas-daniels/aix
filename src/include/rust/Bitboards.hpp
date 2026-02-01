#ifndef Bitboards_HPP
#define Bitboards_HPP

#include "Bitboards.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::Bitboards Bitboards::AsFFI() const {
  return diplomat::capi::Bitboards {
    /* .w_k = */ w_k,
    /* .w_q = */ w_q,
    /* .w_r = */ w_r,
    /* .w_b = */ w_b,
    /* .w_n = */ w_n,
    /* .w_p = */ w_p,
    /* .b_k = */ b_k,
    /* .b_q = */ b_q,
    /* .b_r = */ b_r,
    /* .b_b = */ b_b,
    /* .b_n = */ b_n,
    /* .b_p = */ b_p,
  };
}

inline Bitboards Bitboards::FromFFI(diplomat::capi::Bitboards c_struct) {
  return Bitboards {
    /* .w_k = */ c_struct.w_k,
    /* .w_q = */ c_struct.w_q,
    /* .w_r = */ c_struct.w_r,
    /* .w_b = */ c_struct.w_b,
    /* .w_n = */ c_struct.w_n,
    /* .w_p = */ c_struct.w_p,
    /* .b_k = */ c_struct.b_k,
    /* .b_q = */ c_struct.b_q,
    /* .b_r = */ c_struct.b_r,
    /* .b_b = */ c_struct.b_b,
    /* .b_n = */ c_struct.b_n,
    /* .b_p = */ c_struct.b_p,
  };
}


#endif // Bitboards_HPP
