/*
 * SPDX-FileCopyrightText: 2026 The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

#define LOG_TAG "custom.hardware.biometrics.fingerprint.udfps-service.default"

#include "UdfpsExtension.h"

namespace aidl {
namespace custom {
namespace hardware {
namespace biometrics {
namespace fingerprint {
namespace udfps {

ndk::ScopedAStatus UdfpsExtension::getUdfpsDimZOrder(int32_t z, int32_t* _aidl_return) {
    *_aidl_return = z;
    return ndk::ScopedAStatus::ok();
}

ndk::ScopedAStatus UdfpsExtension::getUdfpsZOrder(int32_t z, bool /*touched*/, int32_t* _aidl_return) {
    *_aidl_return = z;
    return ndk::ScopedAStatus::ok();
}

ndk::ScopedAStatus UdfpsExtension::getUdfpsUsageBits(int64_t usageBits, bool /*touched*/, int64_t* _aidl_return) {
    *_aidl_return = usageBits;
    return ndk::ScopedAStatus::ok();
}

}  // namespace udfps
}  // namespace fingerprint
}  // namespace biometrics
}  // namespace hardware
}  // namespace custom
}  // namespace aidl
