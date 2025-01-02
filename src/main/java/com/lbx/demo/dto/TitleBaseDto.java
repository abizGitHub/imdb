package com.lbx.demo.dto;

import lombok.AllArgsConstructor;
import lombok.Data;

import java.util.List;

@Data
@AllArgsConstructor
public class TitleBaseDto {
    private String titleType;
    private String primaryTitle;
    private String originalTitle;
    private Boolean isAdult;
    private Integer startYear;
    private Integer endYear;
    private Integer runtimeMinute;
    private List<String> genres;
}
