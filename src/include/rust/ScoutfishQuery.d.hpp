#ifndef ScoutfishQuery_D_HPP
#define ScoutfishQuery_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

class DecodeError;
class ScoutfishQueryParseError;


namespace diplomat {
namespace capi {
    struct ScoutfishQuery;
} // namespace capi
} // namespace

class ScoutfishQuery {
public:

  inline static diplomat::result<size_t, ScoutfishQueryParseError> parse_into_bytes(std::string_view s, diplomat::span<uint8_t> out);

  inline static diplomat::result<std::unique_ptr<ScoutfishQuery>, std::monostate> decode_bytes(diplomat::span<const uint8_t> data);

  inline diplomat::result<bool, DecodeError> matches(diplomat::span<const uint8_t> game) const;

  inline diplomat::result<uint32_t, DecodeError> matches_plies(diplomat::span<const uint8_t> game, diplomat::span<uint32_t> out) const;

  inline const diplomat::capi::ScoutfishQuery* AsFFI() const;
  inline diplomat::capi::ScoutfishQuery* AsFFI();
  inline static const ScoutfishQuery* FromFFI(const diplomat::capi::ScoutfishQuery* ptr);
  inline static ScoutfishQuery* FromFFI(diplomat::capi::ScoutfishQuery* ptr);
  inline static void operator delete(void* ptr);
private:
  ScoutfishQuery() = delete;
  ScoutfishQuery(const ScoutfishQuery&) = delete;
  ScoutfishQuery(ScoutfishQuery&&) noexcept = delete;
  ScoutfishQuery operator=(const ScoutfishQuery&) = delete;
  ScoutfishQuery operator=(ScoutfishQuery&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ScoutfishQuery_D_HPP
