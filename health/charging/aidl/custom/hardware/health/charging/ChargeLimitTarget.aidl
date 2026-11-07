/*
 * SPDX-FileCopyrightText: The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.hardware.health.charging;

/** A battery level that the system sets as the limit. */
@VintfStability
parcelable ChargeLimitTarget {
    /** Lowest level a target can set, in percent. */
    const int MIN_LEVEL_PERCENT = 1;
    /** Highest level a target can set, in percent. */
    const int MAX_LEVEL_PERCENT = 99;

    /** Battery level in percent, from MIN_LEVEL_PERCENT to MAX_LEVEL_PERCENT. */
    int levelPercent;
}
