/*
 * SPDX-FileCopyrightText: 2026 The halogenOS Project
 * SPDX-License-Identifier: Apache-2.0
 */

package custom.hardware.biometrics.fingerprint.udfps;

@VintfStability
interface IUdfpsExtension {
    int getUdfpsDimZOrder(int z);
    int getUdfpsZOrder(int z, boolean touched);
    long getUdfpsUsageBits(long usageBits, boolean touched);
}
