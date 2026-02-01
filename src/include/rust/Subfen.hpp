#ifndef Subfen_HPP
#define Subfen_HPP

#include "Subfen.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DecodeError.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct Subfen_parse_result {union {diplomat::capi::Subfen ok; }; bool is_ok;} Subfen_parse_result;
    Subfen_parse_result Subfen_parse(diplomat::capi::DiplomatStringView subfen);
    
    typedef struct Subfen_matches_result {union {bool ok; diplomat::capi::DecodeError err;}; bool is_ok;} Subfen_matches_result;
    Subfen_matches_result Subfen_matches(diplomat::capi::Subfen self, diplomat::capi::DiplomatU8View game);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<Subfen, std::monostate> Subfen::parse(std::string_view subfen) {
  auto result = diplomat::capi::Subfen_parse({subfen.data(), subfen.size()});
  return result.is_ok ? diplomat::result<Subfen, std::monostate>(diplomat::Ok<Subfen>(Subfen::FromFFI(result.ok))) : diplomat::result<Subfen, std::monostate>(diplomat::Err<std::monostate>());
}

inline diplomat::result<bool, DecodeError> Subfen::matches(diplomat::span<const uint8_t> game) {
  auto result = diplomat::capi::Subfen_matches(this->AsFFI(),
    {game.data(), game.size()});
  return result.is_ok ? diplomat::result<bool, DecodeError>(diplomat::Ok<bool>(result.ok)) : diplomat::result<bool, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}


inline diplomat::capi::Subfen Subfen::AsFFI() const {
  return diplomat::capi::Subfen {
    /* .white = */ white,
    /* .black = */ black,
    /* .king = */ king,
    /* .queen = */ queen,
    /* .rook = */ rook,
    /* .bishop = */ bishop,
    /* .knight = */ knight,
    /* .pawn = */ pawn,
  };
}

inline Subfen Subfen::FromFFI(diplomat::capi::Subfen c_struct) {
  return Subfen {
    /* .white = */ c_struct.white,
    /* .black = */ c_struct.black,
    /* .king = */ c_struct.king,
    /* .queen = */ c_struct.queen,
    /* .rook = */ c_struct.rook,
    /* .bishop = */ c_struct.bishop,
    /* .knight = */ c_struct.knight,
    /* .pawn = */ c_struct.pawn,
  };
}


#endif // Subfen_HPP
