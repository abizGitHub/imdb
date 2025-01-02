package com.lbx.demo.model;

import lombok.*;

import java.util.*;
import java.util.stream.Collectors;

/**
 * 2. Return all the titles in which both director and writer are the same person and he/she is still alive
 */
@AllArgsConstructor
@NoArgsConstructor
@ToString
@Getter
@EqualsAndHashCode(onlyExplicitlyIncluded = true)
public class TitleCrew implements FieldSettable {
    private String titleId;
    private final List<String> directors = new ArrayList<>();
    private final List<String> writers = new ArrayList<>();

    public Set<String> sameDirectorAndWriter() {
        return directors.stream().filter(writers::contains).collect(Collectors.toSet());
    }

    @Override
    public void setField(String name, String value) {
        switch (name) {
            case "tconst" -> titleId = value;
            case "directors" -> directors.addAll(Arrays.asList(value.split(",")));
            case "writers" -> writers.addAll(Arrays.asList(value.split(",")));
        }
    }

    public TitleCrew addDirector(String nameId) {
        directors.add(nameId);
        return this;
    }

    public TitleCrew addWriter(String nameId) {
        writers.add(nameId);
        return this;
    }
}

