use oxigraph::sparql::QuerySolutionIter;
use std::{error::Error, fmt};

/// A database that can be spawned and accept SPARQL queries.
pub trait SparqlDBClient {
    fn spawn() -> Self;

    /// Insert graph to the database from a Turtle encoded string.
    fn insert_turtle(&self, turtle: &str) -> Result<(), Box<dyn Error>>;

    /// Make a SPARQL query.
    fn sparql_query(&self, query: &str) -> Result<QuerySolutionIter, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct SparqlQueryError;

impl fmt::Display for SparqlQueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SPARQL query error")
    }
}

impl Error for SparqlQueryError {}
