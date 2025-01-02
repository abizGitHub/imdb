package com.lbx.demo;

import com.jayway.jsonpath.JsonPath;
import com.lbx.demo.controller.CounterFilter;
import com.lbx.demo.model.*;
import com.lbx.demo.service.ImdbService;
import com.lbx.demo.service.NameService;
import com.lbx.demo.service.TitleService;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Order;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.autoconfigure.web.servlet.AutoConfigureMockMvc;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.mock.web.MockMultipartFile;
import org.springframework.test.web.servlet.MockMvc;

import static org.springframework.test.web.servlet.result.MockMvcResultMatchers.*;
import static org.springframework.test.web.servlet.request.MockMvcRequestBuilders.*;

import static org.junit.jupiter.api.Assertions.*;

import org.springframework.http.MediaType;
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders;
import org.springframework.test.web.servlet.setup.MockMvcBuilders;
import org.springframework.web.context.WebApplicationContext;

import java.io.FileInputStream;
import java.nio.charset.StandardCharsets;
import java.util.List;


@AutoConfigureMockMvc
@SpringBootTest
class DemoApplicationTests {

    @Autowired
    private WebApplicationContext webApplicationContext;

    @Autowired
    private MockMvc mockMvc;

    @Autowired
    private TitleService titleService;

    @Autowired
    private NameService nameService;

    @Autowired
    private ImdbService imdbService;

    @BeforeEach
    public void clearAllData() {
        imdbService.clearAllData();
    }

    //1. Import the dataset into the application
    @Order(1)
    @Test
    public void import_the_dataset_into_the_Application() throws Exception {

        var fileNames = List.of(
                "title.basics.tsv",
                "name.basics.tsv",
                "title.principals.tsv",
                "title.crew.tsv",
                "title.ratings.tsv");

        for (String fileName : fileNames) {
            try (FileInputStream fileInputStream = new FileInputStream("src/test/resources/" + fileName)) {

                MockMultipartFile tsvFile = new MockMultipartFile("file", fileName,
                        "multipart/form-data",
                        fileInputStream.readAllBytes());

                MockMvc mockMvc = MockMvcBuilders.webAppContextSetup(webApplicationContext).build();
                mockMvc.perform(MockMvcRequestBuilders.multipart("/api/v1/files").file(tsvFile))
                        .andExpect(status().isOk())
                        .andExpect(content().string(fileName + ":19 rows uploaded!"));
            }
        }

        assertEquals(titleService.getById("tt0556618").getOriginalTitle(), "Forty Steps to Glory");
        assertEquals(nameService.getById("nm0317238").getPrimaryName(), "Gerd Gickel");
        assertEquals(titleService.getByGenre("Western", 0, 200).getTotalRecord(), 5);
    }

    @Order(2)
    @Test
    public void return_titles_in_which_both_director_and_writer_are_the_same_and_alive() throws Exception {
        //********** Arrange
        //====================  create three actors that one of the is dead  ==========================
        nameService.add(new NameBasics("nm0000001", "Marlon Brando", 1924, 2004));
        nameService.add(new NameBasics("nm0000002", "Judith Gick", 1908, null));
        nameService.add(new NameBasics("nm0000003", "John Belushi", 1908, null));
        nameService.add(new NameBasics("nm0000004", "Ingrid Bergman", 1915, 1982));
        //=============================  create  titles   =================================
        titleService.add(new TitleBasics("tt0000001", "short", "Blacksmith Scene",
                "Blacksmith Scene", false, 1893, null, 1));
        titleService.add(new TitleBasics("tt0000002", "tvEpisode", "Fifty Years a Mystery",
                "Fifty Years a Mystery", false, 1957, null, 25));
        titleService.add(new TitleBasics("tt0000003", "tvEpisode", "Forbidden Wedding",
                "Forbidden Wedding", false, 1960, null, 25));
        //=============================  create  crew   =================================
        // same and dead
        imdbService.addTitleCrew(new TitleCrew("tt0000001")
                .addDirector("nm0000001").addDirector("nm0000003")
                .addWriter("nm0000001"));
        // same and alive
        imdbService.addTitleCrew(new TitleCrew("tt0000002")
                .addDirector("nm0000003")
                .addWriter("nm0000003").addWriter("nm0000004"));
        // not same
        imdbService.addTitleCrew(new TitleCrew("tt0000003")
                .addDirector("nm0000001").addDirector("nm0000002")
                .addWriter("nm0000003").addWriter("nm0000004"));

        //********** Act
        String response = mockMvc.perform(get("/api/v1/imdb/titles")
                        .param("sameWriterAndDirectorAndIsAlive", "true")
                        .contentType(MediaType.APPLICATION_JSON))
                .andExpect(status().isOk())
                .andReturn().getResponse().getContentAsString(StandardCharsets.UTF_8);
        //********** Assert
        assertEquals(
                1,
                JsonPath.parse(response).read("$.totalRecord", Integer.class)
        );
        assertEquals(
                "Fifty Years a Mystery",
                JsonPath.parse(response).read("$.content[0].primaryTitle", String.class)
        );
    }

    @Order(3)
    @Test
    public void get_two_actors_and_return_all_the_titles_in_which_both_of_them_played_at() throws Exception {
        //********** Arrange
        //====================  create three actors =================================
        nameService.add(new NameBasics("nm0000001", "Marlon Brando", 1924, 2004));
        nameService.add(new NameBasics("nm0000002", "Judith Gick", 1908, null));
        nameService.add(new NameBasics("nm0000003", "John Belushi", 1908, null));
        nameService.add(new NameBasics("nm0000004", "Ingrid Bergman", 1915, 1982));
        //=============================  create  titles   =================================
        titleService.add(new TitleBasics("tt0000001", "short", "Blacksmith Scene",
                "Blacksmith Scene", false, 1893, null, 1));
        titleService.add(new TitleBasics("tt0000002", "tvEpisode", "Fifty Years a Mystery",
                "Fifty Years a Mystery", false, 1957, null, 25));
        titleService.add(new TitleBasics("tt0000003", "tvEpisode", "Forbidden Wedding",
                "Forbidden Wedding", false, 1960, null, 25));
        titleService.add(new TitleBasics("tt0000004", "movie", "Miss Jerry",
                "Miss Jerry", false, 1969, null, 300));
        //=============================  create  title.principals   =================================
        // both of actors in t1
        imdbService.addTitlePrincipal(
                new TitlePrincipal("tt0000001", "nm0000001", 1, null, "actor", null));
        imdbService.addTitlePrincipal(
                new TitlePrincipal("tt0000001", "nm0000002", 2, null, "actress", null));
        // both of actors in t2
        imdbService.addTitlePrincipal(
                new TitlePrincipal("tt0000002", "nm0000001", 2, null, "actor", null));
        imdbService.addTitlePrincipal(
                new TitlePrincipal("tt0000002", "nm0000002", 2, null, "actress", null));
        // actor1 in t3
        imdbService.addTitlePrincipal(
                new TitlePrincipal("tt0000003", "nm0000001", 1, null, "actor", null));
        // actor2 in t4
        imdbService.addTitlePrincipal(
                new TitlePrincipal("tt0000004", "nm0000002", 2, null, "actress", null));

        //********** Act
        String response = mockMvc.perform(get("/api/v1/imdb/titles")
                        .param("actor1", "Marlon Brando")
                        .param("actor2", "Judith Gick")
                        .contentType(MediaType.APPLICATION_JSON))
                .andExpect(status().isOk())
                .andReturn().getResponse().getContentAsString(StandardCharsets.UTF_8);
        //********** Assert
        assertEquals(
                2,
                JsonPath.parse(response).read("$.totalRecord", Integer.class));

        assertEquals(
                "Fifty Years a Mystery",
                JsonPath.parse(response).read("$.content[0].primaryTitle", String.class));

        assertEquals(
                "Blacksmith Scene",
                JsonPath.parse(response).read("$.content[1].primaryTitle", String.class));
    }

    @Order(4)
    @Test
    public void get_a_genre_from_the_user_and_return_best_titles_on_each_year_for_that_genre_based_on_number_of_votes_and_rating() throws Exception {
        //********** Arrange
        //=============================  create  titles   =================================
        titleService.add(new TitleBasics("tt0000001", "short", "Blacksmith Scene",
                "Blacksmith Scene", false, 2003, 2006, 1)
                .addGenre("Western")
                .addGenre("Action"));
        titleService.add(new TitleBasics("tt0000002", "tvEpisode", "Fifty Years a Mystery",
                "Fifty Years a Mystery", false, 1957, 1982, 25)
                .addGenre("Western")
                .addGenre("Animation"));
        titleService.add(new TitleBasics("tt0000003", "tvEpisode", "Forbidden Wedding",
                "Forbidden Wedding", false, 1960, 1982, 25)
                .addGenre("Western")
                .addGenre("Comedy"));
        titleService.add(new TitleBasics("tt0000004", "movie", "Miss Jerry",
                "Miss Jerry", false, 1980, 1982, 300)
                .addGenre("Western")
                .addGenre("Documentary"));
        //===========================  create  titles.ratings =================================
        imdbService.addTitleRating(
                new TitleRating("tt0000001", 7.9f, 300));//Blacksmith Scene
        imdbService.addTitleRating(
                new TitleRating("tt0000002", 6.4f, 100));//Fifty Years a Mystery
        imdbService.addTitleRating(
                new TitleRating("tt0000003", 6.4f, 5591));//Forbidden Wedding
        imdbService.addTitleRating(
                new TitleRating("tt0000004", 8.3f, 2));//Miss Jerry
        //********** Act
        String response = mockMvc.perform(get("/api/v1/imdb/titles/year")
                        .param("genre", "Western")
                        .contentType(MediaType.APPLICATION_JSON))
                .andExpect(status().isOk())
                .andReturn().getResponse().getContentAsString(StandardCharsets.UTF_8);

        //********** Assert
        assertEquals(
                4,
                JsonPath.parse(response).read("$.totalRecord", Integer.class));

        //============ all in year 2006 =========
        assertEquals(
                2006,
                JsonPath.parse(response).read("$.content[0].year", Integer.class));
        assertEquals(
                "Blacksmith Scene",
                JsonPath.parse(response).read("$.content[0].titles[0].primaryTitle", String.class));
        //============ all in year 1982 =========
        assertEquals(
                1982,
                JsonPath.parse(response).read("$.content[1].year", Integer.class));
        assertEquals(
                "Miss Jerry",
                JsonPath.parse(response).read("$.content[1].titles[0].primaryTitle", String.class));
        assertEquals(
                "Forbidden Wedding",
                JsonPath.parse(response).read("$.content[1].titles[1].primaryTitle", String.class));
        assertEquals(
                "Fifty Years a Mystery",
                JsonPath.parse(response).read("$.content[1].titles[2].primaryTitle", String.class));
    }

    /***
     *  we can also use "spring actuator" to count all and every request to the app or a specific path
     **/
    @Order(5)
    @Test
    public void count_how_many_HTTP_requests_you_received_in_this_application_since_the_last_startup() throws Exception {
        var beforeCall = CounterFilter.appCallCounter.get();
        for (int i = 0; i < 7; i++) {
            mockMvc.perform(get("/api/v1/imdb/titles/year")
                    .param("genre", "Western")
                    .contentType(MediaType.APPLICATION_JSON));

        }
        assertEquals(
                7,
                CounterFilter.appCallCounter.get() - beforeCall);
    }

    @Test
    public void importFileWithWrongName() throws Exception {
        //========================== file name matters ========================

        MockMultipartFile file = new MockMultipartFile("file", "some-random-name.tsv",
                "multipart/form-data", "some tsv".getBytes());

        MockMvc mockMvc = MockMvcBuilders.webAppContextSetup(webApplicationContext).build();
        mockMvc.perform(MockMvcRequestBuilders.multipart("/api/v1/files").file(file))
                .andExpect(status().isBadRequest())
                .andExpect(content()
                        .string("wrong file: some-random-name.tsv\n"
                                + " valid names:[title.basics.tsv, name.basics.tsv, title.principals.tsv, title.crew.tsv, title.ratings.tsv]"
                        ));
    }

    @Test
    void testTitlesWithMissingActor() throws Exception {
        // Arrange

        // Act & Assert
        mockMvc.perform(get("/api/v1/imdb/titles")
                        .param("actor2", "some-actor")
                        .param("sameWriterAndDirectorAndIsAlive", "false")
                        .contentType(MediaType.APPLICATION_JSON))
                .andExpect(status().isBadRequest())
                .andExpect(content().string("actor1 and actor2 could not be empty."));
    }


}
