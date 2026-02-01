#ifndef MoveDetails_HPP
#define MoveDetails_HPP

#include "MoveDetails.d.hpp"

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


inline diplomat::capi::MoveDetails MoveDetails::AsFFI() const {
  return diplomat::capi::MoveDetails {
    /* .ply = */ ply,
    /* .role = */ role,
    /* .from = */ from,
    /* .to = */ to,
    /* .capture = */ capture,
    /* .is_castle = */ is_castle,
    /* .promotion = */ promotion,
    /* .is_check = */ is_check,
    /* .is_checkmate = */ is_checkmate,
    /* .is_en_passant = */ is_en_passant,
  };
}

inline MoveDetails MoveDetails::FromFFI(diplomat::capi::MoveDetails c_struct) {
  return MoveDetails {
    /* .ply = */ c_struct.ply,
    /* .role = */ c_struct.role,
    /* .from = */ c_struct.from,
    /* .to = */ c_struct.to,
    /* .capture = */ c_struct.capture,
    /* .is_castle = */ c_struct.is_castle,
    /* .promotion = */ c_struct.promotion,
    /* .is_check = */ c_struct.is_check,
    /* .is_checkmate = */ c_struct.is_checkmate,
    /* .is_en_passant = */ c_struct.is_en_passant,
  };
}


#endif // MoveDetails_HPP
