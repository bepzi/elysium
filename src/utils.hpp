#pragma once

#define ELYSIUM_DISABLE_COPY(_ClassName)                                                           \
    _ClassName(const _ClassName &) = delete;                                                       \
    _ClassName &operator=(const _ClassName &) = delete;

#define ELYSIUM_DISABLE_MOVE(_ClassName)                                                           \
    _ClassName(_ClassName &&) = delete;                                                            \
    _ClassName &operator=(_ClassName &&) = delete;

#define ELYSIUM_DISABLE_COPY_MOVE(_ClassName)                                                      \
public:                                                                                            \
    ELYSIUM_DISABLE_COPY(_ClassName)                                                               \
    ELYSIUM_DISABLE_MOVE(_ClassName)                                                               \
private:
