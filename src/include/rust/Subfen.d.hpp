#ifndef Subfen_D_HPP
#define Subfen_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class DecodeError;


namespace diplomat {
namespace capi {
    struct Subfen {
      uint64_t white;
      uint64_t black;
      uint64_t king;
      uint64_t queen;
      uint64_t rook;
      uint64_t bishop;
      uint64_t knight;
      uint64_t pawn;
    };
    
    typedef struct Subfen_option {union { Subfen ok; }; bool is_ok; } Subfen_option;
} // namespace capi
} // namespace


struct Subfen {
  uint64_t white;
  uint64_t black;
  uint64_t king;
  uint64_t queen;
  uint64_t rook;
  uint64_t bishop;
  uint64_t knight;
  uint64_t pawn;

  inline static diplomat::result<Subfen, std::monostate> parse(std::string_view subfen);

  inline diplomat::result<bool, DecodeError> matches(diplomat::span<const uint8_t> game);

  inline diplomat::capi::Subfen AsFFI() const;
  inline static Subfen FromFFI(diplomat::capi::Subfen c_struct);
};


#endif // Subfen_D_HPP
