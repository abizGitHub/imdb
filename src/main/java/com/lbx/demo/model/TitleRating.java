package com.lbx.demo.model;

import lombok.*;

/**
 * 4. Get a genre from the user and return best titles on each year for that genre based on number of votes and rating
 */
@NoArgsConstructor
@AllArgsConstructor
@ToString
@Getter
@EqualsAndHashCode(onlyExplicitlyIncluded = true)
public class TitleRating implements FieldSettable {
    private String titleId;
    private Float averageRating;
    private Integer numVotes;

    @Override
    public void setField(String name, String value) {
        switch (name) {
            case "tconst" -> titleId = value;
            case "numVotes" -> numVotes = Integer.valueOf(value);
            case "averageRating" -> averageRating = Float.valueOf(value);
        }
    }
}
