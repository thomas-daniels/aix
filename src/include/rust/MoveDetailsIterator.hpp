#ifndef MoveDetailsIterator_HPP
#define MoveDetailsIterator_HPP

#include "MoveDetailsIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DecodeError.hpp"
#include "MoveDetails.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct MoveDetailsIterator_next_result {union {diplomat::capi::MoveDetails ok; diplomat::capi::DecodeError err;}; bool is_ok;} MoveDetailsIterator_next_result;
    MoveDetailsIterator_next_result MoveDetailsIterator_next(diplomat::capi::MoveDetailsIterator* self);
    
    typedef struct MoveDetailsIterator_nth_result {union {diplomat::capi::MoveDetails ok; diplomat::capi::DecodeError err;}; bool is_ok;} MoveDetailsIterator_nth_result;
    MoveDetailsIterator_nth_result MoveDetailsIterator_nth(diplomat::capi::MoveDetailsIterator* self, int16_t n);
    
    
    void MoveDetailsIterator_destroy(MoveDetailsIterator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<MoveDetails, DecodeError> MoveDetailsIterator::next() {
  auto result = diplomat::capi::MoveDetailsIterator_next(this->AsFFI());
  return result.is_ok ? diplomat::result<MoveDetails, DecodeError>(diplomat::Ok<MoveDetails>(MoveDetails::FromFFI(result.ok))) : diplomat::result<MoveDetails, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<MoveDetails, DecodeError> MoveDetailsIterator::nth(int16_t n) {
  auto result = diplomat::capi::MoveDetailsIterator_nth(this->AsFFI(),
    n);
  return result.is_ok ? diplomat::result<MoveDetails, DecodeError>(diplomat::Ok<MoveDetails>(MoveDetails::FromFFI(result.ok))) : diplomat::result<MoveDetails, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline const diplomat::capi::MoveDetailsIterator* MoveDetailsIterator::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::MoveDetailsIterator*>(this);
}

inline diplomat::capi::MoveDetailsIterator* MoveDetailsIterator::AsFFI() {
  return reinterpret_cast<diplomat::capi::MoveDetailsIterator*>(this);
}

inline const MoveDetailsIterator* MoveDetailsIterator::FromFFI(const diplomat::capi::MoveDetailsIterator* ptr) {
  return reinterpret_cast<const MoveDetailsIterator*>(ptr);
}

inline MoveDetailsIterator* MoveDetailsIterator::FromFFI(diplomat::capi::MoveDetailsIterator* ptr) {
  return reinterpret_cast<MoveDetailsIterator*>(ptr);
}

inline void MoveDetailsIterator::operator delete(void* ptr) {
  diplomat::capi::MoveDetailsIterator_destroy(reinterpret_cast<diplomat::capi::MoveDetailsIterator*>(ptr));
}


#endif // MoveDetailsIterator_HPP
