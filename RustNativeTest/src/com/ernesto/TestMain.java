package com.ernesto;

import java.util.*;

public class TestMain {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        String str = sc.nextLine();
        String key = sc.nextLine();
        RustFeature rf = new RustFeature();
        rf.setStr(str);
        rf.setKey(key);
        rf.setReverse(true);
        System.out.println(rf.encrypt());
    }
}
