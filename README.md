# IMDB
 - simple Spring-Boot application that imports dataset of IMDB and serve some APIs without any database
 - all imported data is stored in memory (Java object collections) and after restarting the app importing dataset is needed.
  

##### 1- user imports file using the path below 

  curl -F "file=@YOUR_LOCAL_DIR/name.basics.tsv" localhost:8080/api/v1/files
  
 IMDB dataset that used in app include : 

* title.basics.tsv
* name.basics.tsv
* title.principals.tsv
* title.crew.tsv
* title.ratings.tsv  

 IMPORTANT : name of file matters 
 
 TSVMapper reads lines of tsv file and calls corresponding method to converting it to a java object by calling setField() of data-class.
 to retrieve data efficiently record are stored in TreeMap which has binary-search for its key-set.

##### 2- to `Return all the titles in which both director and writer are the same person and he/she is still alive`

   curl 'localhost:8080/api/v1/imdb/titles?sameWriterAndDirectorAndIsAlive=true' 

##### 3- to `Get two actors and return all the titles in which both of them played at`
   
   curl 'localhost:8080/api/v1/imdb/titles?actor1=name1&actor2=name2'

##### 4- to `Get a genre from the user and return best titles on each year for that genre based on number of votes and rating`
  
   curl 'localhost:8080/api/v1/imdb/titles/year?genre=Short&page=1&size=1'  
  
##### 5- to `Count how many HTTP requests you received in this application since the last startup`
 
   use spring actuator
<br/>
- all functionalities are unit-tested in : DemoApplicationTests

 
