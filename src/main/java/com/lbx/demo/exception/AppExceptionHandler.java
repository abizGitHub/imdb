package com.lbx.demo.exception;

import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;

import java.util.List;

@ControllerAdvice
public class AppExceptionHandler {
    @ExceptionHandler(InvalidFileNameException.class)
    public ResponseEntity<String> handleInvalidFileNameException(InvalidFileNameException ex) {
        var validNames = List.of(
                "title.basics.tsv",
                "name.basics.tsv",
                "title.principals.tsv",
                "title.crew.tsv",
                "title.ratings.tsv");
        return new ResponseEntity<>("wrong file: " + ex.getMessage() + "\n valid names:" + validNames, HttpStatus.BAD_REQUEST);
    }

    @ExceptionHandler(Exception.class)
    public ResponseEntity<String> handleException(Exception ex) {
        return new ResponseEntity<>("An error occurred:" + ex.getMessage(), HttpStatus.INTERNAL_SERVER_ERROR);
    }

    @ExceptionHandler(NameNotFound.class)
    public ResponseEntity<String> handleResourceNotFound(NameNotFound ex) {
        return new ResponseEntity<>("name not found:" + ex.getMessage(), HttpStatus.NOT_FOUND);
    }

    @ExceptionHandler(GenreNotFound.class)
    public ResponseEntity<String> handleGenreNotFound(GenreNotFound ex) {
        return new ResponseEntity<>("genre not found:" + ex.getMessage(), HttpStatus.NOT_FOUND);
    }

    @ExceptionHandler(BadRequestException.class)
    public ResponseEntity<String> handleBadRequestException(BadRequestException ex) {
        return new ResponseEntity<>( ex.getMessage(), HttpStatus.BAD_REQUEST);
    }
}
