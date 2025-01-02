package com.lbx.demo.service;

import com.lbx.demo.exception.InvalidFileNameException;
import com.lbx.demo.model.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.io.InputStreamReader;

@Service
public class FileService {
    private static final Logger logger = LoggerFactory.getLogger(FileService.class);

    private final ImdbService imdbService;

    public FileService(ImdbService imdbService) {
        this.imdbService = imdbService;
    }

    public int save(MultipartFile file) throws IOException {
        var rowCount = 0;
        try (InputStreamReader reader = new InputStreamReader(file.getInputStream())) {
            switch (file.getOriginalFilename()) {
                case "title.basics.tsv" ->
                        rowCount = new TSVMapper<>(TitleBasics.class, reader).writeTo(imdbService::addTitleBasics);
                case "name.basics.tsv" ->
                        rowCount = new TSVMapper<>(NameBasics.class, reader).writeTo(imdbService::addNameBasics);
                case "title.principals.tsv" ->
                        rowCount = new TSVMapper<>(TitlePrincipal.class, reader).writeTo(imdbService::addTitlePrincipal);
                case "title.crew.tsv" ->
                        rowCount = new TSVMapper<>(TitleCrew.class, reader).writeTo(imdbService::addTitleCrew);
                case "title.ratings.tsv" ->
                        rowCount = new TSVMapper<>(TitleRating.class, reader).writeTo(imdbService::addTitleRating);
                default -> {
                    logger.error("Undefined file name.");
                    throw new InvalidFileNameException(file.getOriginalFilename());
                }
            }
        }
        return rowCount;
    }
}
