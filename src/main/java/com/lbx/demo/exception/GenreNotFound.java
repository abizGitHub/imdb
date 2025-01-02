package com.lbx.demo.exception;

public class GenreNotFound extends RuntimeException {
    public GenreNotFound(String msg) {
        super(msg);
    }
}
