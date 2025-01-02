package com.lbx.demo.dto;

import lombok.AllArgsConstructor;
import lombok.Data;

import java.util.List;

@Data
@AllArgsConstructor
public class TitleByYearDto {
    private Integer year;
    private List<TitleBaseDto> titles;
}
