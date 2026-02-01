#ifndef MoveDetails_D_HPP
#define MoveDetails_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct MoveDetails {
      uint16_t ply;
      int8_t role;
      uint8_t from;
      uint8_t to;
      int8_t capture;
      bool is_castle;
      int8_t promotion;
      bool is_check;
      bool is_checkmate;
      bool is_en_passant;
    };
    
    typedef struct MoveDetails_option {union { MoveDetails ok; }; bool is_ok; } MoveDetails_option;
} // namespace capi
} // namespace


struct MoveDetails {
  uint16_t ply;
  int8_t role;
  uint8_t from;
  uint8_t to;
  int8_t capture;
  bool is_castle;
  int8_t promotion;
  bool is_check;
  bool is_checkmate;
  bool is_en_passant;

  inline diplomat::capi::MoveDetails AsFFI() const;
  inline static MoveDetails FromFFI(diplomat::capi::MoveDetails c_struct);
};


#endif // MoveDetails_D_HPP
