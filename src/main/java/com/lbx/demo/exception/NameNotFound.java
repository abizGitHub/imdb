package com.lbx.demo.exception;

public class NameNotFound extends RuntimeException {
    public NameNotFound(String msg) {
        super(msg);
    }
}
