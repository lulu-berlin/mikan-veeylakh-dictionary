use oxigraph::sparql::QuerySolutionIter;
use std::{error::Error, fmt};

/// A database that can be spawned and accept SPARQL queries.
pub trait SparqlDBClient {
    fn spawn() -> Self;
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
