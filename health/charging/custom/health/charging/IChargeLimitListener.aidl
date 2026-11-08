/*
 * SPDX-FileCopyrightText: The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.health.charging;

/** @hide */
oneway interface IChargeLimitListener {
    /** The charge limit changed. IChargingControlManager.getLimit returns the new one. */
    void onLimitChanged();
}
