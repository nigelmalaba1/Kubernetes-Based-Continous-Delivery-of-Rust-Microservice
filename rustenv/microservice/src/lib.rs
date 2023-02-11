/*Create a Microservice that will pull Netflix API for movies list

//use actix_web::{web, App, HttpResponse, HttpServer, Responder};
/*use serde::{Deserialize, Serialize};
use std::collections::HashMap;*/

//[derive(Serialize, Deserialize)]
struct Movie {
    id: i32,
    title: String,
    year: i32,
    rating: f32,
}

//write a function that calls the Netflix API and returns a list of movies
fn callmovies() -> Vec<Movie> {
    //call the Netflix API
    //return a list of movies

    let mut movies = Vec::new();
    movies.push(Movie {
        id: 1,
        title: "The Matrix".to_string(),
        year: 1999,
        rating: 8.7,
    });
    movies.push(Movie {
        id: 2,
        title: "The Matrix Reloaded".to_string(),
        year: 2003,
        rating: 7.2,
    });
    movies.push(Movie {
        id: 3,
        title: "The Matrix Revolutions".to_string(),
        year: 2003,
        rating: 6.7,
    });

    //return the list of movies
    return movies;
}
*/