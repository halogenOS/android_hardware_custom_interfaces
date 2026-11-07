/*
 * SPDX-FileCopyrightText: The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.hardware.health.charging;

import custom.hardware.health.charging.ChargeLimitTarget;

/** Receives the targets of a flow. Only the latest target counts. */
@VintfStability
oneway interface IChargeLimitCollector {
    /** Limits the battery to the target. A null target means normal charging. */
    void emit(in @nullable ChargeLimitTarget target);
}
