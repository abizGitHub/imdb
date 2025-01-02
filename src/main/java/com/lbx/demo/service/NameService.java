package com.lbx.demo.service;

import com.lbx.demo.exception.NameNotFound;
import com.lbx.demo.model.NameBasics;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Map;
import java.util.TreeMap;

@Service
public class NameService {
    private static final Logger logger = LoggerFactory.getLogger(NameService.class);

    private final Map<String, NameBasics> namesTree = new TreeMap<>();
    private final Map<String, NameBasics> primaryNameTree = new TreeMap<>();

    public void add(NameBasics nameBasic) {
        if (nameBasic.getPrimaryName() != null) {
            primaryNameTree.put(nameBasic.getPrimaryName(), nameBasic);
            namesTree.put(nameBasic.getId(), nameBasic);
        } else {
            logger.warn("nameBasics has null primaryName:" + nameBasic);
        }
    }

    public NameBasics getById(String id) {
        return namesTree.get(id);
    }

    public NameBasics getByPrimaryName(String primaryName) {
        var nb = primaryNameTree.get(primaryName);
        if (nb == null) throw new NameNotFound(primaryName);
        return nb;
    }

    public void clearAllData() {
        primaryNameTree.clear();
        namesTree.clear();
    }
}
