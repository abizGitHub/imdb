package com.lbx.demo.controller;

import org.springframework.stereotype.Component;
import jakarta.servlet.Filter;
import jakarta.servlet.FilterChain;
import jakarta.servlet.ServletException;
import jakarta.servlet.ServletRequest;
import jakarta.servlet.ServletResponse;

import java.io.IOException;
import java.util.concurrent.atomic.AtomicLong;


@Component
public class CounterFilter implements Filter {

    public static AtomicLong appCallCounter = new AtomicLong(0);

    @Override
    public void doFilter(ServletRequest servletRequest, ServletResponse servletResponse, FilterChain filterChain) throws IOException, ServletException {
        appCallCounter.incrementAndGet();
        filterChain.doFilter(servletRequest, servletResponse);
    }
}
