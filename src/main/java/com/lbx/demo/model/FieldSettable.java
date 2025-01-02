package com.lbx.demo.model;

public interface FieldSettable {
    // its compile-time process and very faster than reflection field-setting.
    void setField(String name, String value);
}