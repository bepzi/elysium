#pragma once

/*!
 * \brief Forbids copy-construction and copy-assignment.
 *
 * Prefer putting this macro in the `public:` section of your class in
 * order to get better compiler errors when these functions are
 * accidentally used. ("Attempting to reference deleted function"
 * vs. "Cannot reference private member")
 */
#define ELYSIUM_DISABLE_COPY(_ClassName)                                                           \
    _ClassName(const _ClassName &) = delete;                                                       \
    _ClassName &operator=(const _ClassName &) = delete;

/*!
 * \brief Forbids move-construction and move-assignment.
 *
 * Prefer putting this macro in the `public:` section of your class in
 * order to get better compiler errors when these functions are
 * accidentally used. ("Attempting to reference deleted function"
 * vs. "Cannot reference private member")
 */
#define ELYSIUM_DISABLE_MOVE(_ClassName)                                                           \
    _ClassName(_ClassName &&) = delete;                                                            \
    _ClassName &operator=(_ClassName &&) = delete;

/*!
 * \brief Forbids copying or moving.
 *
 * Use this for classes that should always be accessed behind some
 * form of indirection, like a pointer or reference, and for classes
 * that should always maintain their unique memory address once
 * created.
 *
 * Prefer putting this macro in the `public:` section of your class.
 */
#define ELYSIUM_DISABLE_COPY_MOVE(_ClassName)                                                      \
    ELYSIUM_DISABLE_COPY(_ClassName)                                                               \
    ELYSIUM_DISABLE_MOVE(_ClassName)

/*!
 * \brief Hints the compiler that a certain statement is likely, or
 * unlikely to be executed.
 *
 * Useful for guiding compiler optimizations in highly
 * performance-sensitive code. Most of the time though, you don't need
 * to use this.
 */
#if defined(__GNUC__) || defined(__clang__) || defined(__INTEL_COMPILER)
#    define ELYSIUM_LIKELY(_x) __builtin_expect(!!(_x), 1)
#    define ELYSIUM_UNLIKELY(_x) __builtin_expect(!!(_x), 0)
#else // Compiler probably doesn't support __builtin_expect
#    define ELYSIUM_LIKELY(_x) _x
#    define ELYSIUM_UNLIKELY(_x) _x
#endif
