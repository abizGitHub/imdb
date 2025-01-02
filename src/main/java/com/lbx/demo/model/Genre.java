package com.lbx.demo.model;
/**
 * to generate all enums simply run:
 * 1- cat title.basics.tsv | awk -F'\t' '{print $9}' | tr -s ',' '\n' | sort | uniq
 * 2- cat genre | tr 'a-z' 'A-Z'
 **/
public enum Genre {
    ACTION("Action"),
    ADULT("Adult"),
    ADVENTURE("Adventure"),
    ANIMATION("Animation"),
    BIOGRAPHY("Biography"),
    COMEDY("Comedy"),
    CRIME("Crime"),
    DOCUMENTARY("Documentary"),
    DRAMA("Drama"),
    FAMILY("Family"),
    FANTASY("Fantasy"),
    FILM_NOIR("Film-Noir"),
    GAME_SHOW("Game-Show"),
    GENRES("genres"),
    HISTORY("History"),
    HORROR("Horror"),
    MUSIC("Music"),
    MUSICAL("Musical"),
    MYSTERY("Mystery"),
    NEWS("News"),
    REALITY_TV("Reality-TV"),
    ROMANCE("Romance"),
    SCI_FI("Sci-Fi"),
    SHORT("Short"),
    SPORT("Sport"),
    TALK_SHOW("Talk-Show"),
    THRILLER("Thriller"),
    WAR("War"),
    WESTERN("Western");

    Genre(String name) {
    }
}
