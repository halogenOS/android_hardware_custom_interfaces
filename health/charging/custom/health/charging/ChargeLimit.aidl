/*
 * SPDX-FileCopyrightText: The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.health.charging;

/** @hide */
parcelable ChargeLimit {
    /** Battery level charging stops at, in percent. */
    int levelPercent;

    /**
     * Width of the band above levelPercent, in percent. The battery stays
     * between levelPercent and levelPercent plus variationPercent.
     */
    int variationPercent;
}
