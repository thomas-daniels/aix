#ifndef Game_HPP
#define Game_HPP

#include "Game.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "Bitboards.hpp"
#include "DecodeError.hpp"
#include "MoveDetailsIterator.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct Game_from_bytes_result {union {diplomat::capi::Game* ok; diplomat::capi::DecodeError err;}; bool is_ok;} Game_from_bytes_result;
    Game_from_bytes_result Game_from_bytes(diplomat::capi::DiplomatU8View data);
    
    typedef struct Game_pieces_at_position_result {union {diplomat::capi::Bitboards ok; diplomat::capi::DecodeError err;}; bool is_ok;} Game_pieces_at_position_result;
    Game_pieces_at_position_result Game_pieces_at_position(diplomat::capi::DiplomatU8View data, int32_t pos);
    
    typedef struct Game_board_at_position_result {union { diplomat::capi::DecodeError err;}; bool is_ok;} Game_board_at_position_result;
    Game_board_at_position_result Game_board_at_position(diplomat::capi::DiplomatU8View data, int32_t pos, diplomat::capi::DiplomatI8ViewMut out);
    
    typedef struct Game_fen_at_position_result {union { diplomat::capi::DecodeError err;}; bool is_ok;} Game_fen_at_position_result;
    Game_fen_at_position_result Game_fen_at_position(diplomat::capi::DiplomatU8View data, int32_t pos, diplomat::capi::DiplomatWrite* write);
    
    typedef struct Game_to_uci_string_result {union { diplomat::capi::DecodeError err;}; bool is_ok;} Game_to_uci_string_result;
    Game_to_uci_string_result Game_to_uci_string(diplomat::capi::DiplomatU8View data, diplomat::capi::DiplomatWrite* write);
    
    typedef struct Game_to_pgn_string_result {union { diplomat::capi::DecodeError err;}; bool is_ok;} Game_to_pgn_string_result;
    Game_to_pgn_string_result Game_to_pgn_string(diplomat::capi::DiplomatU8View data, diplomat::capi::DiplomatWrite* write);
    
    typedef struct Game_moved_pieces_result {union { diplomat::capi::DecodeError err;}; bool is_ok;} Game_moved_pieces_result;
    Game_moved_pieces_result Game_moved_pieces(diplomat::capi::DiplomatU8View data, diplomat::capi::DiplomatWrite* write);
    
    typedef struct Game_recompress_result {union {size_t ok; diplomat::capi::DecodeError err;}; bool is_ok;} Game_recompress_result;
    Game_recompress_result Game_recompress(diplomat::capi::DiplomatU8View data, uint8_t level, diplomat::capi::DiplomatU8ViewMut out);
    
    diplomat::capi::MoveDetailsIterator* Game_move_details_iterator(const diplomat::capi::Game* self);
    
    
    void Game_destroy(Game* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Game>, DecodeError> Game::from_bytes(diplomat::span<const uint8_t> data) {
  auto result = diplomat::capi::Game_from_bytes({data.data(), data.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<Game>, DecodeError>(diplomat::Ok<std::unique_ptr<Game>>(std::unique_ptr<Game>(Game::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Game>, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<Bitboards, DecodeError> Game::pieces_at_position(diplomat::span<const uint8_t> data, int32_t pos) {
  auto result = diplomat::capi::Game_pieces_at_position({data.data(), data.size()},
    pos);
  return result.is_ok ? diplomat::result<Bitboards, DecodeError>(diplomat::Ok<Bitboards>(Bitboards::FromFFI(result.ok))) : diplomat::result<Bitboards, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, DecodeError> Game::board_at_position(diplomat::span<const uint8_t> data, int32_t pos, diplomat::span<int8_t> out) {
  auto result = diplomat::capi::Game_board_at_position({data.data(), data.size()},
    pos,
    {out.data(), out.size()});
  return result.is_ok ? diplomat::result<std::monostate, DecodeError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<std::string, DecodeError> Game::fen_at_position(diplomat::span<const uint8_t> data, int32_t pos) {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Game_fen_at_position({data.data(), data.size()},
    pos,
    &write);
  return result.is_ok ? diplomat::result<std::string, DecodeError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<std::string, DecodeError> Game::to_uci_string(diplomat::span<const uint8_t> data) {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Game_to_uci_string({data.data(), data.size()},
    &write);
  return result.is_ok ? diplomat::result<std::string, DecodeError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<std::string, DecodeError> Game::to_pgn_string(diplomat::span<const uint8_t> data) {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Game_to_pgn_string({data.data(), data.size()},
    &write);
  return result.is_ok ? diplomat::result<std::string, DecodeError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<std::string, DecodeError> Game::moved_pieces(diplomat::span<const uint8_t> data) {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::Game_moved_pieces({data.data(), data.size()},
    &write);
  return result.is_ok ? diplomat::result<std::string, DecodeError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline diplomat::result<size_t, DecodeError> Game::recompress(diplomat::span<const uint8_t> data, uint8_t level, diplomat::span<uint8_t> out) {
  auto result = diplomat::capi::Game_recompress({data.data(), data.size()},
    level,
    {out.data(), out.size()});
  return result.is_ok ? diplomat::result<size_t, DecodeError>(diplomat::Ok<size_t>(result.ok)) : diplomat::result<size_t, DecodeError>(diplomat::Err<DecodeError>(DecodeError::FromFFI(result.err)));
}

inline std::unique_ptr<MoveDetailsIterator> Game::move_details_iterator() const {
  auto result = diplomat::capi::Game_move_details_iterator(this->AsFFI());
  return std::unique_ptr<MoveDetailsIterator>(MoveDetailsIterator::FromFFI(result));
}

inline const diplomat::capi::Game* Game::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Game*>(this);
}

inline diplomat::capi::Game* Game::AsFFI() {
  return reinterpret_cast<diplomat::capi::Game*>(this);
}

inline const Game* Game::FromFFI(const diplomat::capi::Game* ptr) {
  return reinterpret_cast<const Game*>(ptr);
}

inline Game* Game::FromFFI(diplomat::capi::Game* ptr) {
  return reinterpret_cast<Game*>(ptr);
}

inline void Game::operator delete(void* ptr) {
  diplomat::capi::Game_destroy(reinterpret_cast<diplomat::capi::Game*>(ptr));
}


#endif // Game_HPP
