/*
 * SPDX-FileCopyrightText: The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.hardware.health.charging;

import custom.hardware.health.charging.IChargeLimitCollector;

/** The targets of the system, published to the HAL. */
@VintfStability
interface IChargeLimitFlow {
    /** Emits the current target to the collector first, then every change. */
    oneway void collect(IChargeLimitCollector collector);
}
