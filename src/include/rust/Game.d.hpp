#ifndef Game_D_HPP
#define Game_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct MoveDetailsIterator; }
class MoveDetailsIterator;
struct Bitboards;
class DecodeError;


namespace diplomat {
namespace capi {
    struct Game;
} // namespace capi
} // namespace

class Game {
public:

  inline static diplomat::result<std::unique_ptr<Game>, DecodeError> from_bytes(diplomat::span<const uint8_t> data);

  inline static diplomat::result<Bitboards, DecodeError> pieces_at_position(diplomat::span<const uint8_t> data, int32_t pos);

  inline static diplomat::result<std::monostate, DecodeError> board_at_position(diplomat::span<const uint8_t> data, int32_t pos, diplomat::span<int8_t> out);

  inline static diplomat::result<std::string, DecodeError> fen_at_position(diplomat::span<const uint8_t> data, int32_t pos);

  inline static diplomat::result<std::string, DecodeError> to_uci_string(diplomat::span<const uint8_t> data);

  inline static diplomat::result<std::string, DecodeError> to_pgn_string(diplomat::span<const uint8_t> data);

  inline static diplomat::result<std::string, DecodeError> moved_pieces(diplomat::span<const uint8_t> data);

  inline static diplomat::result<size_t, DecodeError> recompress(diplomat::span<const uint8_t> data, uint8_t level, diplomat::span<uint8_t> out);

  inline std::unique_ptr<MoveDetailsIterator> move_details_iterator() const;

  inline const diplomat::capi::Game* AsFFI() const;
  inline diplomat::capi::Game* AsFFI();
  inline static const Game* FromFFI(const diplomat::capi::Game* ptr);
  inline static Game* FromFFI(diplomat::capi::Game* ptr);
  inline static void operator delete(void* ptr);
private:
  Game() = delete;
  Game(const Game&) = delete;
  Game(Game&&) noexcept = delete;
  Game operator=(const Game&) = delete;
  Game operator=(Game&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Game_D_HPP
