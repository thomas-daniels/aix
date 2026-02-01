#ifndef MoveDetailsIterator_D_HPP
#define MoveDetailsIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

struct MoveDetails;
class DecodeError;


namespace diplomat {
namespace capi {
    struct MoveDetailsIterator;
} // namespace capi
} // namespace

class MoveDetailsIterator {
public:

  inline diplomat::result<MoveDetails, DecodeError> next();

  inline diplomat::result<MoveDetails, DecodeError> nth(int16_t n);

  inline const diplomat::capi::MoveDetailsIterator* AsFFI() const;
  inline diplomat::capi::MoveDetailsIterator* AsFFI();
  inline static const MoveDetailsIterator* FromFFI(const diplomat::capi::MoveDetailsIterator* ptr);
  inline static MoveDetailsIterator* FromFFI(diplomat::capi::MoveDetailsIterator* ptr);
  inline static void operator delete(void* ptr);
private:
  MoveDetailsIterator() = delete;
  MoveDetailsIterator(const MoveDetailsIterator&) = delete;
  MoveDetailsIterator(MoveDetailsIterator&&) noexcept = delete;
  MoveDetailsIterator operator=(const MoveDetailsIterator&) = delete;
  MoveDetailsIterator operator=(MoveDetailsIterator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // MoveDetailsIterator_D_HPP
