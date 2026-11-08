/*
 * Copyright (C) 2026 The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.health.charging;

import custom.health.charging.ChargeLimit;
import custom.health.charging.IChargeLimitListener;

/** @hide */
interface IChargingControlManager {
    boolean isSupported();

    /** The charge limit the user set, or null when there is none. */
    @nullable ChargeLimit getLimit();

    /**
     * Stores limit, or removes the charge limit when limit is null. Returns
     * false when the band of limit lies outside the levels a charge limit
     * allows, or when storing it failed. The caller needs the
     * WRITE_SECURE_SETTINGS permission.
     */
    boolean setLimit(in @nullable ChargeLimit limit);

    /** Calls listener after each change of the charge limit. */
    void registerListener(IChargeLimitListener listener);

    void unregisterListener(IChargeLimitListener listener);
}
