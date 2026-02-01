#ifndef DecodeError_HPP
#define DecodeError_HPP

#include "DecodeError.d.hpp"

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

inline diplomat::capi::DecodeError DecodeError::AsFFI() const {
  return static_cast<diplomat::capi::DecodeError>(value);
}

inline DecodeError DecodeError::FromFFI(diplomat::capi::DecodeError c_enum) {
  switch (c_enum) {
    case diplomat::capi::DecodeError_NoErrorNoValue:
    case diplomat::capi::DecodeError_EmptyBlob:
    case diplomat::capi::DecodeError_InvalidCompressionLevel:
    case diplomat::capi::DecodeError_InvalidEncodedGameConstructionData:
    case diplomat::capi::DecodeError_InvalidDataDuringDecoding:
      return static_cast<DecodeError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // DecodeError_HPP
