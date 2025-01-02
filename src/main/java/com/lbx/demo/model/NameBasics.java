package com.lbx.demo.model;


import lombok.*;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

@AllArgsConstructor
@NoArgsConstructor
@ToString
@Getter
@EqualsAndHashCode(onlyExplicitlyIncluded = true)
public class NameBasics implements FieldSettable {
    @EqualsAndHashCode.Include
    private String id;
    private String primaryName;
    private Integer birthYear;
    private Integer deathYear;
    private final List<String> primaryProfession = new ArrayList<>();
    private final List<String> knownForTitles = new ArrayList<>();

    @Override
    public void setField(String name, String value) {
        switch (name) {
            case "nconst" -> id = value;
            case "primaryName" -> primaryName = value;
            case "birthYear" -> birthYear = Integer.valueOf(value);
            case "deathYear" -> deathYear = Integer.valueOf(value);
            case "primaryProfession" -> primaryProfession.addAll(Arrays.asList(value.split(",")));
            case "knownForTitles" -> knownForTitles.addAll(Arrays.asList(value.split(",")));
        }
    }

}
