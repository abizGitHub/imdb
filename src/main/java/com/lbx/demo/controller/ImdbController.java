package com.lbx.demo.controller;

import com.lbx.demo.dto.Page;
import com.lbx.demo.dto.TitleBaseDto;
import com.lbx.demo.dto.TitleByYearDto;
import com.lbx.demo.exception.BadRequestException;
import com.lbx.demo.service.ImdbService;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/${app.version}/imdb")
public class ImdbController {

    private final ImdbService imdbService;

    public ImdbController(ImdbService imdbService) {
        this.imdbService = imdbService;
    }

    @GetMapping("/titles")
    public Page<TitleBaseDto> titles(
            @RequestParam(required = false, defaultValue = "") String actor1,
            @RequestParam(required = false, defaultValue = "") String actor2,
            @RequestParam(required = false, defaultValue = "false") Boolean sameWriterAndDirectorAndIsAlive,
            @RequestParam(defaultValue = "10") int size,
            @RequestParam(defaultValue = "0") int page) {

        if (sameWriterAndDirectorAndIsAlive) {
            return imdbService.titlesWithSameCrewAndAlive(size, page);
        } else {
            if (actor1.isBlank() || actor2.isBlank())
                throw new BadRequestException("actor1 and actor2 could not be empty.");
            return imdbService.commonTitles(actor1, actor2, size, page);
        }
    }

    @GetMapping("/titles/year")
    public Page<TitleByYearDto> ratingByGenre(
            @RequestParam String genre,
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size) {
        return imdbService.ratingByGenre(genre, page, size);
    }

}


