#ifndef DecodeError_D_HPP
#define DecodeError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum DecodeError {
      DecodeError_NoErrorNoValue = 0,
      DecodeError_EmptyBlob = 1,
      DecodeError_InvalidCompressionLevel = 2,
      DecodeError_InvalidEncodedGameConstructionData = 3,
      DecodeError_InvalidDataDuringDecoding = 4,
    };
    
    typedef struct DecodeError_option {union { DecodeError ok; }; bool is_ok; } DecodeError_option;
} // namespace capi
} // namespace

class DecodeError {
public:
  enum Value {
    NoErrorNoValue = 0,
    EmptyBlob = 1,
    InvalidCompressionLevel = 2,
    InvalidEncodedGameConstructionData = 3,
    InvalidDataDuringDecoding = 4,
  };

  DecodeError() = default;
  // Implicit conversions between enum and ::Value
  constexpr DecodeError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::DecodeError AsFFI() const;
  inline static DecodeError FromFFI(diplomat::capi::DecodeError c_enum);
private:
    Value value;
};


#endif // DecodeError_D_HPP
