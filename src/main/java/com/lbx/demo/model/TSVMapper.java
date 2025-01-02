package com.lbx.demo.model;


import java.io.BufferedReader;
import java.io.IOException;
import java.io.Reader;
import java.util.*;
import java.util.function.Consumer;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class TSVMapper<T extends FieldSettable> {

    private static final Logger logger = LoggerFactory.getLogger(TSVMapper.class);

    private Class<T> clazz;
    private BufferedReader reader;
    private ArrayList<String> tsvHeaders = new ArrayList<>();

    public TSVMapper(Class<T> clazz, Reader reader) {
        this.clazz = clazz;
        this.reader = new BufferedReader(reader);
    }

    public int writeTo(Consumer<T> consumer) throws IOException {
        var count = 0;
        tsvHeaders.addAll(Arrays.asList(reader.readLine().split("\t")));
        String row;
        while ((row = reader.readLine()) != null) {
            consumer.accept(readAndConvertRow(row));
            count++;
        }
        return count;
    }

    private T readAndConvertRow(String row) {
        T instance;
        try {
            instance = clazz.getConstructor().newInstance();
            var rowSplitted = row.split("\t");
            for (int i = 0; i < tsvHeaders.size(); i++) {
                var columnValue = rowSplitted[i];
                columnValue = (columnValue.equals("\\N")) ? null : columnValue.trim();
                if (columnValue == null)
                    continue;
                instance.setField(tsvHeaders.get(i), columnValue);
            }
        } catch (Exception e) {
            logger.error("error while converting tsv row:", e);
            return null;
        }
        return instance;
    }
}

