/*
 * SPDX-FileCopyrightText: The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.hardware.health.charging;

import custom.hardware.health.charging.IChargeLimitFlow;

/**
 * Limits the battery to the level that the system sets. The device
 * implementation selects the hardware for that.
 */
@VintfStability
interface IChargeLimit {
    /**
     * Attaches the flow of targets to the HAL. The system calls it once the HAL
     * is registered, and again after every restart of the HAL. The HAL collects
     * the flow and returns the hardware to normal charging when the flow dies.
     */
    void attach(IChargeLimitFlow flow);
}
