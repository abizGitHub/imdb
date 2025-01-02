package com.lbx.demo.model;

import com.lbx.demo.dto.TitleBaseDto;
import lombok.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * 4. Get a genre from the user and return best titles on each year for that genre based on number of votes and rating
 */
@AllArgsConstructor
@NoArgsConstructor
@ToString
@Getter
@EqualsAndHashCode(onlyExplicitlyIncluded = true)
public class TitleBasics implements FieldSettable {
    @EqualsAndHashCode.Include
    private String id;
    private String titleType;
    private String primaryTitle;
    private String originalTitle;
    private Boolean isAdult;
    private Integer startYear;
    private Integer endYear;
    private Integer runtimeMinutes;
    private final List<String> genres = new ArrayList<>();

    public Integer getYear() {
        return (endYear == null) ? startYear : endYear;
    }

    public TitleBaseDto toDto() {
        return new TitleBaseDto(titleType, primaryTitle, originalTitle, isAdult, startYear, endYear, runtimeMinutes, genres);
    }

    @Override
    public void setField(String name, String value) {
        switch (name) {
            case "tconst" -> id = value;
            case "titleType" -> titleType = value;
            case "primaryTitle" -> primaryTitle = value;
            case "originalTitle" -> originalTitle = value;
            case "isAdult" -> isAdult = (value.equals("1"));
            case "startYear" -> startYear = Integer.valueOf(value);
            case "endYear" -> endYear = Integer.valueOf(value);
            case "runtimeMinutes" -> runtimeMinutes = Integer.valueOf(value);
            case "genres" -> genres.addAll(Arrays.asList(value.split(",")));
        }
    }

    public TitleBasics addGenre(String genre) {
        genres.add(genre);
        return this;
    }
}
