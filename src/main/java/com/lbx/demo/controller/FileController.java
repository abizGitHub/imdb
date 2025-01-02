package com.lbx.demo.controller;

import com.lbx.demo.service.FileService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;

@RestController
@RequestMapping("/api/${app.version}/files")
public class FileController {
    private static final Logger logger = LoggerFactory.getLogger(FileController.class);

    private final FileService fileService;

    public FileController(FileService uploadService) {
        this.fileService = uploadService;
    }

    @PostMapping(consumes = {MediaType.MULTIPART_FORM_DATA_VALUE})
    public ResponseEntity<String> upload(@RequestParam("file") MultipartFile file) throws IOException {
        int rows = fileService.save(file);
        var msg = file.getOriginalFilename() + ":" + rows + " rows uploaded!";
        logger.info(msg);
        return ResponseEntity.ok(msg);
    }

}

