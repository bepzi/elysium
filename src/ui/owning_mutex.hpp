#pragma once

#include "utils.hpp"

#include <mutex>
#include <optional>

namespace elysium {

template<typename T, typename Lockable = std::mutex>
class MutexGuard;

/*!
 * A mutex that owns the data it protects, forcing you to lock it
 * before you can access the data.
 *
 * This class is similar in spirit to Rust's
 * [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html),
 * albeit with much weaker safety guarantees. The usual C++ mutex
 * safety rules apply: don't delete a locked mutex, and don't leak
 * references to the guarded data.
 */
template<typename T, typename Lockable = std::mutex>
class OwningMutex
{
public:
    ELYSIUM_DISABLE_COPY_MOVE(OwningMutex)

    explicit OwningMutex(T &&value = T()) : value(std::move(value)) { }

    ~OwningMutex() = default;

    [[nodiscard]] MutexGuard<T, Lockable> lock()
    {
        mutex.lock();
        return { &value, &mutex };
    }

    [[nodiscard]] std::optional<MutexGuard<T, Lockable>> try_lock()
    {
        if (!mutex.try_lock())
            return std::nullopt;
        return std::make_optional<MutexGuard<T, Lockable>>({ &value, &mutex });
    }

private:
    T value;
    Lockable mutex;
};

/*!
 * RAII lock holder, created by locking an `OwningMutex`.
 *
 * When this object is destroyed, the mutex will be unlocked. As such,
 * don't allow it to outlive the `OwningMutex` that created it, and
 * don't leak references to the guarded data.
 *
 * This class is similar in spirit to Rust's
 * [`MutexGuard`](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html),
 * albeit with much weaker safety guarantees.
 */
template<typename T, typename Lockable>
class MutexGuard
{
public:
    ELYSIUM_DISABLE_COPY(MutexGuard)
    MutexGuard &operator=(MutexGuard &&) = delete;

    ~MutexGuard()
    {
        if (mutex != nullptr)
            mutex->unlock();
    }

    MutexGuard(MutexGuard &&other) noexcept : value(nullptr), mutex(nullptr)
    {
        std::swap(value, other.value);
        std::swap(mutex, other.mutex);
    }

    [[nodiscard]] const T &get() const { return *value; }

    [[nodiscard]] T &getMut() const { return *value; }

private:
    friend OwningMutex<T, Lockable>;

    MutexGuard(T *value, Lockable *mutex) : value(value), mutex(mutex) { }

    T *value;
    Lockable *mutex;
};

} // namespace elysium
