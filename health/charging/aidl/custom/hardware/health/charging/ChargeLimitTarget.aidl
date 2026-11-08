/*
 * SPDX-FileCopyrightText: The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.hardware.health.charging;

/**
 * A battery level that the system sets as the limit, and the band above it in
 * which the battery may move.
 */
@VintfStability
parcelable ChargeLimitTarget {
    /** Lowest level a target can set, in percent. */
    const int MIN_LEVEL_PERCENT = 1;
    /** Highest level a target can set, in percent. */
    const int MAX_LEVEL_PERCENT = 99;

    /** Battery level in percent, from MIN_LEVEL_PERCENT to MAX_LEVEL_PERCENT. */
    int levelPercent;

    /**
     * Width of the band above levelPercent, in percent, from 0 to 100 minus
     * levelPercent. The battery stays between levelPercent and levelPercent
     * plus variationPercent. The HAL sets how the level moves inside the
     * band. A value of 0 means the battery stays at levelPercent. A target
     * sent by a version 1 system has variationPercent 0. A HAL reads a value
     * outside this range as 0.
     */
    int variationPercent = 0;
}
