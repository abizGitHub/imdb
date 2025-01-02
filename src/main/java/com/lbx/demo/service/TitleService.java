package com.lbx.demo.service;

import com.lbx.demo.dto.Page;
import com.lbx.demo.exception.GenreNotFound;
import com.lbx.demo.model.TitleBasics;
import com.lbx.demo.model.TitleRating;
import org.springframework.stereotype.Service;

import java.util.*;

@Service
public class TitleService {

    private final Map<String, TitleBasics> titlesTree = new TreeMap<>();
    private final Map<String, List<TitleBasics>> titlesByGenre = new HashMap<>();
    private final Map<String, TitleRating> ratingTree = new TreeMap<>();

    public void add(TitleBasics title) {
        titlesTree.put(title.getId(), title);
        List<String> genres = title.getGenres();
        if (genres == null)
            return;
        genres.forEach(g -> {
            titlesByGenre.putIfAbsent(g, new ArrayList<>());
            titlesByGenre.get(g).add(title);
        });
    }

    public TitleBasics getById(String id) {
        return titlesTree.get(id);
    }

    public Page<TitleBasics> getByGenre(String genre, int page, int size) {
        List<TitleBasics> titles = titlesByGenre.get(genre);
        if (titles == null)
            throw new GenreNotFound(genre);

        int startIndex = page * size;
        int endIndex = Math.min(startIndex + size, titles.size());

        Comparator<TitleRating> ratingComparator =
                Comparator.comparing(TitleRating::getAverageRating)
                        .thenComparing(TitleRating::getNumVotes).reversed();

        return new Page(titles.stream().sorted(
                        (t1, t2) -> {
                            var r1 = ratingTree.get(t1.getId());
                            var r2 = ratingTree.get(t2.getId());
                            if (r1 == null)
                                return -1;
                            if (r2 == null)
                                return 1;
                            return ratingComparator.compare(r1, r2);
                        }
                ).skip(startIndex)
                .limit(endIndex - startIndex)
                .toList(),
                titles.size()
        );
    }

    public void addTitleRating(TitleRating data) {
        ratingTree.put(data.getTitleId(), data);
    }

    public void clearAllData() {
        ratingTree.clear();
        titlesByGenre.clear();
        titlesTree.clear();
    }
}
