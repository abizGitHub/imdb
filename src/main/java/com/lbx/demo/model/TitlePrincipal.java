package com.lbx.demo.model;

import lombok.*;


/**
* 3. Get two actors and return all the titles in which both of them played at
**/
@AllArgsConstructor
@NoArgsConstructor
@ToString
@Getter
@EqualsAndHashCode(onlyExplicitlyIncluded = true)
public class TitlePrincipal implements FieldSettable{
    private String titleId;
    private String nameId;
    private Integer ordering;
    private String category;
    private String job;
    private String characters;

    public boolean isActor() {
        return job.equals("actress") || job.equals("actor");
    }

    @Override
    public void setField(String name, String value) {
        switch (name) {
            case "tconst" -> titleId = value;
            case "nconst" -> nameId = value;
            case "ordering" -> ordering = Integer.valueOf(value);
            case "category" -> category = value;
            case "job" -> job = value;
            case "characters" -> characters = value;
        }
    }
}
