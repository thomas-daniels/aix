#ifndef ScoutfishQueryParseError_D_HPP
#define ScoutfishQueryParseError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum ScoutfishQueryParseError {
      ScoutfishQueryParseError_InvalidPiece = 1,
      ScoutfishQueryParseError_InvalidImbalanceFormat = 2,
      ScoutfishQueryParseError_InvalidMaterialFormat = 3,
      ScoutfishQueryParseError_InvalidSideToMove = 4,
      ScoutfishQueryParseError_InvalidSan = 5,
      ScoutfishQueryParseError_InvalidSyntaxOrStructure = 6,
      ScoutfishQueryParseError_BincodeError = 7,
      ScoutfishQueryParseError_BufferTooSmall = 8,
      ScoutfishQueryParseError_CursorWriteError = 9,
    };
    
    typedef struct ScoutfishQueryParseError_option {union { ScoutfishQueryParseError ok; }; bool is_ok; } ScoutfishQueryParseError_option;
} // namespace capi
} // namespace

class ScoutfishQueryParseError {
public:
  enum Value {
    InvalidPiece = 1,
    InvalidImbalanceFormat = 2,
    InvalidMaterialFormat = 3,
    InvalidSideToMove = 4,
    InvalidSan = 5,
    InvalidSyntaxOrStructure = 6,
    BincodeError = 7,
    BufferTooSmall = 8,
    CursorWriteError = 9,
  };

  ScoutfishQueryParseError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ScoutfishQueryParseError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::ScoutfishQueryParseError AsFFI() const;
  inline static ScoutfishQueryParseError FromFFI(diplomat::capi::ScoutfishQueryParseError c_enum);
private:
    Value value;
};


#endif // ScoutfishQueryParseError_D_HPP
