#ifndef ScoutfishQuery_HPP
#define ScoutfishQuery_HPP

#include "ScoutfishQuery.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DecodeError.hpp"
#include "ScoutfishQueryParseError.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ScoutfishQuery_parse_into_bytes_result {union {size_t ok; diplomat::capi::ScoutfishQueryParseError err;}; bool is_ok;} ScoutfishQuery_parse_into_bytes_result;
    ScoutfishQuery_parse_into_bytes_result ScoutfishQuery_parse_into_bytes(diplomat::capi::DiplomatStringView s, diplomat::capi::DiplomatU8ViewMut out);
    
    typedef struct ScoutfishQuery_decode_bytes_result {union {diplomat::capi::ScoutfishQuery* ok; }; bool is_ok;} ScoutfishQuery_decode_bytes_result;
    ScoutfishQuery_decode_bytes_result ScoutfishQuery_decode_bytes(diplomat::capi::DiplomatU8View data);
    
    typedef struct ScoutfishQuery_matches_result {union {bool ok; diplomat::capi::DecodeError err;}; bool is_ok;} ScoutfishQuery_matches_result;
    ScoutfishQuery_matches_result ScoutfishQuery_matches(const diplomat::capi::ScoutfishQuery* self, diplomat::capi::DiplomatU8View game);
    
    typedef struct ScoutfishQuery_matches_plies_result {union {uint32_t ok; diplomat::capi::DecodeError err;}; bool is_ok;} ScoutfishQuery_matches_plies_result;
    ScoutfishQuery_matches_plies_result ScoutfishQuery_matches_plies(const diplomat::capi::ScoutfishQuery* self, diplomat::capi::DiplomatU8View game, diplomat::capi::DiplomatU32ViewMut out);
    
    
    void ScoutfishQuery_destroy(ScoutfishQuery* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<size_t, ScoutfishQueryParseError> ScoutfishQuery::parse_into_bytes(std::string_view s, diplomat::span<uint8_t> out) {
  auto result = diplomat::capi::ScoutfishQuery_parse_into_bytes({s.data(), s.size()},
    {out.data(), out.size()});
  return result.is_ok ? diplomat::result<size_t, ScoutfishQueryParseError>(diplomat::Ok<size_t>(result.ok)) : diplomat::result<size_t, ScoutfishQueryParseError>(diplomat::Err<ScoutfishQueryParseError>(ScoutfishQueryParseError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ScoutfishQuery>, std::monostate> ScoutfishQuery::decode_bytes(diplomat::span<const uint8_t> data) {
  auto result = diplomat::capi::ScoutfishQuery_decode_bytes({data.data(), data.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<ScoutfishQuery>, std::monostate>(diplomat::Ok<std::unique_ptr<ScoutfishQuery>>(std::unique_ptr<ScoutfishQuery>(ScoutfishQuery::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ScoutfishQuery>, std::monostate>(diplomat::Err<std::monostate>());
}

inline diplomat::result<bool, DecodeError> ScoutfishQuery::matches(diplomat::span<const uint8_t> game) const {
  auto result = diplomat::capi::ScoutfishQuery_matches(this->AsFFI(),
    {game.data(), game.size()});
  return result.is_ok ? diplomat::result<bool, DecodeError>(diplomat::Ok<bool>(result.ok)) : diplomat::result<bool, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<uint32_t, DecodeError> ScoutfishQuery::matches_plies(diplomat::span<const uint8_t> game, diplomat::span<uint32_t> out) const {
  auto result = diplomat::capi::ScoutfishQuery_matches_plies(this->AsFFI(),
    {game.data(), game.size()},
    {out.data(), out.size()});
  return result.is_ok ? diplomat::result<uint32_t, DecodeError>(diplomat::Ok<uint32_t>(result.ok)) : diplomat::result<uint32_t, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline const diplomat::capi::ScoutfishQuery* ScoutfishQuery::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ScoutfishQuery*>(this);
}

inline diplomat::capi::ScoutfishQuery* ScoutfishQuery::AsFFI() {
  return reinterpret_cast<diplomat::capi::ScoutfishQuery*>(this);
}

inline const ScoutfishQuery* ScoutfishQuery::FromFFI(const diplomat::capi::ScoutfishQuery* ptr) {
  return reinterpret_cast<const ScoutfishQuery*>(ptr);
}

inline ScoutfishQuery* ScoutfishQuery::FromFFI(diplomat::capi::ScoutfishQuery* ptr) {
  return reinterpret_cast<ScoutfishQuery*>(ptr);
}

inline void ScoutfishQuery::operator delete(void* ptr) {
  diplomat::capi::ScoutfishQuery_destroy(reinterpret_cast<diplomat::capi::ScoutfishQuery*>(ptr));
}


#endif // ScoutfishQuery_HPP
