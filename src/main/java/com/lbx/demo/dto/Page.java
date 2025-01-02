package com.lbx.demo.dto;

import lombok.AllArgsConstructor;
import lombok.Data;

import java.util.List;

@Data
@AllArgsConstructor
public class Page<T> {
    private List<T> content;
    private int totalRecord;

    public static Page empty() {
        return new Page(null, 0);
    }
}
