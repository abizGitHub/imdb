package com.lbx.demo.service;

import com.lbx.demo.dto.Page;
import com.lbx.demo.dto.TitleBaseDto;
import com.lbx.demo.dto.TitleByYearDto;
import com.lbx.demo.model.*;
import org.springframework.stereotype.Service;

import java.util.*;
import java.util.stream.Collectors;

@Service
public class ImdbService {

    private final TitleService titleService;
    private final NameService nameService;
    private final Map<String, List<TitlePrincipal>> principalByNameIdTree = new TreeMap<>();
    private final List<TitleCrew> crewList = new ArrayList<>();

    public ImdbService(TitleService titleService, NameService nameService) {
        this.titleService = titleService;
        this.nameService = nameService;
    }

    public Page<TitleBaseDto> titlesWithSameCrewAndAlive(int size, int page) {
        Set<TitleBasics> titles = crewList.stream()
                .filter(c -> {
                    var sameCrew = c.sameDirectorAndWriter();
                    if (sameCrew == null) return false;
                    return sameCrew.stream().anyMatch(n -> {
                        NameBasics crewName = nameService.getById(n);
                        return crewName != null && crewName.getDeathYear() == null;
                    });
                })
                .map(c -> titleService.getById(c.getTitleId()))
                .collect(Collectors.toSet());

        int startIndex = page * size;
        long endIndex = Math.min(startIndex + size, titles.size());

        return new Page<>(
                titles.stream()
                        .skip(startIndex)
                        .limit(endIndex - startIndex)
                        .map(TitleBasics::toDto)
                        .collect(Collectors.toList()),
                titles.size()
        );
    }

    public Page<TitleBaseDto> commonTitles(String actor1, String actor2, int size, int page) {
        List<TitlePrincipal> principals1 = principalByNameIdTree.get(
                nameService.getByPrimaryName(actor1).getId());

        List<TitlePrincipal> principals2 = principalByNameIdTree.get(
                nameService.getByPrimaryName(actor2).getId());

        if (principals1 == null || principals2 == null)
            return Page.empty();

        Set<String> titles1 = principals1.stream()
                .filter(TitlePrincipal::isActor)
                .map(TitlePrincipal::getTitleId)
                .collect(Collectors.toSet());

        Set<String> titles2 = principals2.stream()
                .filter(TitlePrincipal::isActor)
                .map(TitlePrincipal::getTitleId)
                .collect(Collectors.toSet());

        Set<TitleBasics> sharedTitles = titles1.stream()
                .filter(titles2::contains)
                .map(titleService::getById)
                .collect(Collectors.toSet());

        int startIndex = page * size;
        long endIndex = Math.min(startIndex + size, sharedTitles.size());

        return new Page<>(
                sharedTitles.stream()
                        .skip(startIndex)
                        .limit(endIndex - startIndex)
                        .map(TitleBasics::toDto)
                        .collect(Collectors.toList()),
                sharedTitles.size()
        );
    }

    public Page<TitleByYearDto> ratingByGenre(String genre, int page, int size) {
        Page<TitleBasics> titles = titleService.getByGenre(genre, page, size);
        Map<Integer, List<TitleBasics>> tiltesByYearMap =
                titles.getContent()
                        .stream()
                        .collect(Collectors.groupingBy(TitleBasics::getYear));

        var years = tiltesByYearMap.keySet().stream().sorted(Comparator.reverseOrder());

        return new Page<>(
                years.map(year ->
                        new TitleByYearDto(year, tiltesByYearMap.get(year).stream().map(TitleBasics::toDto).toList())
                ).toList(),
                titles.getTotalRecord());
    }

    public void addTitleBasics(TitleBasics data) {
        titleService.add(data);
    }

    public void addNameBasics(NameBasics data) {
        nameService.add(data);
    }

    public void addTitlePrincipal(TitlePrincipal data) {
        principalByNameIdTree.putIfAbsent(data.getNameId(), new ArrayList<>());
        principalByNameIdTree.get(data.getNameId()).add(data);
    }

    public void addTitleCrew(TitleCrew data) {
        crewList.add(data);
    }

    public void addTitleRating(TitleRating data) {
        titleService.addTitleRating(data);
    }

    public void clearAllData() {
        titleService.clearAllData();
        nameService.clearAllData();
        crewList.clear();
        principalByNameIdTree.clear();
    }
}
