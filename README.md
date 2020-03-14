# Server
Author:  Nick Robinson  (nro2@pdx.edu)
Project Name: Server

### Overview
This project is a backend and database for a web app that stores faculty members and their committee assignments, as well as other various pieces of info about them.

Currently it is strictly a backend, but it has 13 endpoints written.  It currently has 3 deletes, 3 inserts, and 6 gets, along with a splash page that lets the user know that the app is running.

The database is Postgresql and is stored in a Docker container.

The intended purpose of this project was to strengthen my abilities not only with Rust, but with REST APIs. 

### Dependencies:

* Rust (must be a nightly version for Rocket)

* Docker

* Diesel

* Rocket

* Chrono

* Serde


### Getting started:

To start the database, run 

``` docker-compose up```

in the root directory with docker running.

Then start the server: 
```cargo run```

Once the server and the database are both running, navigate to http://localhost:8080/.

### Endpoints

Currently, the following endpoints are available:
* http://localhost:8080/committees
  This gets a list of all committees and their information.
* http://localhost:8080/committees/<id>
  This searches for a specific committee that matches the particular ID.  The database is set up with IDs from 1-10.
* http://localhost:8080/faculty
   This gets a list of all of the faculty and their information in the database.
* http://localhost:8080/faculty/<email>
  This matches a particular faculty member by email. A list of emails is available in the seeding information in the init.sql file, but one for immediate testing is wolsborn@pdx.edu.
* http://localhost:8080/department
  Retrieves a list of all departments and their information.
* http://localhost:8080/department<id>
  Gets a specific department by ID.  Currently the database is set up with departments IDs from 1-5.
  
  Also if you are using a program like Postman to hit the endpoints, deletes are available for all of these endpoints as well as inserts.  The insert takes a JSON, and the deletes work by searching for the primary keys listed in the specific gets above.
  
  Currently, the delete is not functioning properly because I did not set up cascading deletes, so it gives foreign key violations.  This will be addressed in the future.
  
  Also of note, the insert is not fully working as intended because for some reason I could not get the serial primary keys to increment.  The insert does work if you specify a value for the primary key, which is not ideal, but I plan to address this in the future.

### Testing

The testing that I did was all manual.  I started with the get endpoints which I could see the results in the browser.  To test my delete and insert, I used Postman.  This allowed me to insert into various tables and then delete them, as well as get them.

I was not sure how to automate the testing for the endpoints, so manual was the route that I went.

### What Worked

Rocket was great.  Once I got it functioning, it worked exactly as intended.   Diesel was a little tougher as I had no experience with an ORM, but as soon as I figured out how to use it, it was great also.  

The database in the docker container was something I had experience with already, so it worked great for me as well.

### What didn't work

As stated above, I was not able to get the delete functioning fully, and the insert does not work as intended.  These are two things that I do plan to fix in the future though, because I plan to continue working on this over spring break.  

### How satisfied am I with the result?

I am pretty satisfied. It is a functioning web server in a language that I don't have a ton of experience with, so getting it to run and do what I wanted to do was satisfying.  

### What would I like to improve in the future?

I would like to add more endpoints and eventually a front end to it, as well as figure out automated testing eventually.  I would also really like to figure out the insert and delete, but I think I have a great baseline for a full stack web app that will be a lot of fun to continue working on.
  
  

