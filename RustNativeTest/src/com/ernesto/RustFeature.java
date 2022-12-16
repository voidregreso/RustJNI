package com.ernesto;

public class RustFeature {

    static {
        System.loadLibrary("RustJNILib");
    }
    private boolean reverse;
    private String str, key;

    public boolean isReverse() {
        return reverse;
    }

    public void setReverse(boolean reverse) {
        this.reverse = reverse;
    }

    public String getStr() {
        return str;
    }

    public void setStr(String str) {
        this.str = str;
    }

    public String getKey() {
        return key;
    }

    public void setKey(String key) {
        this.key = key;
    }

    public String encrypt() {
        byte[] val = cryptRC4(str.getBytes(), key.getBytes(), reverse);
        String result = "";
        for(byte ch : val) {
            result += String.format("%X", ch) + " ";
        }
        return result;
    }

    public native byte[] cryptRC4(byte[] cipher, byte[] key, boolean reverse);
}
