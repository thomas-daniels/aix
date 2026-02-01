#ifndef BITS_HPP
#define BITS_HPP

#if _cplusplus >= 201907L
#include <bit>
#else
#include <cstdint>
#include <type_traits>
#include <limits>
#include <climits>
#endif

namespace bits {

#if _cplusplus >= 201907L
template <typename T>
constexpr int countr_zero(T x) noexcept {
	return std::countr_zero(x);
}

template <typename T>
constexpr int popcount(T x) noexcept {
	return std::popcount(x);
}
#else

// Helper to get bit width of type T
template <typename T>
constexpr int bit_width = sizeof(T) * CHAR_BIT;

// Fallback implementation for popcount using bit manipulation
template <typename T>
constexpr int popcount_fallback(T x) noexcept {
	int count = 0;
	while (x != 0) {
		count += x & 1;
		x >>= 1;
	}
	return count;
}

// Fallback implementation for countr_zero using bit manipulation
template <typename T>
constexpr int countr_zero_fallback(T x) noexcept {
	if (x == 0)
		return bit_width<T>;
	int count = 0;
	while ((x & 1) == 0) {
		count++;
		x >>= 1;
	}
	return count;
}

// Main popcount function using intrinsics where available
template <typename T>
constexpr int popcount(T x) noexcept {
	if constexpr (std::is_unsigned_v<T>) {
#if defined(__GNUC__) || defined(__clang__)
		if constexpr (sizeof(T) <= sizeof(unsigned int)) {
			return __builtin_popcount(x);
		} else if constexpr (sizeof(T) <= sizeof(unsigned long)) {
			return __builtin_popcountl(x);
		} else if constexpr (sizeof(T) <= sizeof(unsigned long long)) {
			return __builtin_popcountll(x);
		}
#elif defined(_MSC_VER)
		if constexpr (sizeof(T) <= sizeof(unsigned int)) {
			return __popcnt(x);
		} else if constexpr (sizeof(T) <= sizeof(unsigned long long)) {
			return __popcnt64(x);
		}
#endif
		return popcount_fallback(x);
	} else {
		static_assert(std::is_unsigned_v<T>, "popcount is only defined for unsigned integer types");
	}
}

// Main countr_zero function using intrinsics where available
template <typename T>
constexpr int countr_zero(T x) noexcept {
	if constexpr (std::is_unsigned_v<T>) {
#if defined(__GNUC__) || defined(__clang__)
		if constexpr (sizeof(T) <= sizeof(unsigned int)) {
			return x == 0 ? bit_width<T> : __builtin_ctz(x);
		} else if constexpr (sizeof(T) <= sizeof(unsigned long)) {
			return x == 0 ? bit_width<T> : __builtin_ctzl(x);
		} else if constexpr (sizeof(T) <= sizeof(unsigned long long)) {
			return x == 0 ? bit_width<T> : __builtin_ctzll(x);
		}
#elif defined(_MSC_VER)
		if constexpr (sizeof(T) <= sizeof(unsigned int)) {
			unsigned long index;
			return _BitScanForward(&index, x) ? index : bit_width<T>;
		} else if constexpr (sizeof(T) <= sizeof(unsigned long long)) {
			unsigned long index;
			return _BitScanForward64(&index, x) ? index : bit_width<T>;
		}
#endif
		return countr_zero_fallback(x);
	} else {
		static_assert(std::is_unsigned_v<T>, "countr_zero is only defined for unsigned integer types");
	}
}

#endif // _cplusplus >= 201907L

} // namespace bits

#endif // BITS_HPP