/*
 * Copyright (C) 2026 The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.health.charging;

/** @hide */
interface IChargingControlManager {
    /** Key of the Settings.System entry that contains the charge limit in percent. */
    const String LIMIT_SETTING = "charging_limit";

    /** Value of LIMIT_SETTING for no limit, the largest int. */
    const int NO_LIMIT = 2147483647;

    boolean isSupported();
}
