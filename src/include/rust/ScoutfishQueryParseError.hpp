#ifndef ScoutfishQueryParseError_HPP
#define ScoutfishQueryParseError_HPP

#include "ScoutfishQueryParseError.d.hpp"

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

inline diplomat::capi::ScoutfishQueryParseError ScoutfishQueryParseError::AsFFI() const {
  return static_cast<diplomat::capi::ScoutfishQueryParseError>(value);
}

inline ScoutfishQueryParseError ScoutfishQueryParseError::FromFFI(diplomat::capi::ScoutfishQueryParseError c_enum) {
  switch (c_enum) {
    case diplomat::capi::ScoutfishQueryParseError_InvalidPiece:
    case diplomat::capi::ScoutfishQueryParseError_InvalidImbalanceFormat:
    case diplomat::capi::ScoutfishQueryParseError_InvalidMaterialFormat:
    case diplomat::capi::ScoutfishQueryParseError_InvalidSideToMove:
    case diplomat::capi::ScoutfishQueryParseError_InvalidSan:
    case diplomat::capi::ScoutfishQueryParseError_InvalidSyntaxOrStructure:
    case diplomat::capi::ScoutfishQueryParseError_BincodeError:
    case diplomat::capi::ScoutfishQueryParseError_BufferTooSmall:
    case diplomat::capi::ScoutfishQueryParseError_CursorWriteError:
      return static_cast<ScoutfishQueryParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ScoutfishQueryParseError_HPP
