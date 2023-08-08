package com.bxyz.mpc;

public class Native {
    static {
        System.loadLibrary("multi_party_ecdsa");
    }

    public static native String createKey(final int number, final String address, final String room);

    public static native String signData(final String address, final String room, final int[] parties, final String data_to_sign, final String local_share);
}
