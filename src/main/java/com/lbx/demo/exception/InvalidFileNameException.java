package com.lbx.demo.exception;

public class InvalidFileNameException extends RuntimeException {
    public InvalidFileNameException(String msg) {
        super(msg);
    }
}
