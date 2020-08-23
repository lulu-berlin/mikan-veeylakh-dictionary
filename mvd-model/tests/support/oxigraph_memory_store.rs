use super::sparql::{SparqlDBClient, SparqlQueryError};
use oxigraph::{
    sparql::{QueryOptions, QueryResults, QueryResultsFormat, QuerySolutionIter},
    MemoryStore,
};

pub(crate) struct OxigraphMemoryStore(MemoryStore);

impl SparqlDBClient for OxigraphMemoryStore {
    fn spawn() -> Self {
        Self(MemoryStore::new())
    }

    fn sparql_query(&self, query: &str) -> Result<QuerySolutionIter, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        self.0
            .query(query, QueryOptions::default())?
            .write(&mut results, QueryResultsFormat::Xml)?;

        if let QueryResults::Solutions(solutions) =
            QueryResults::read(std::io::Cursor::new(results), QueryResultsFormat::Xml)?
        {
            Ok(solutions)
        } else {
            Err(Box::new(SparqlQueryError))
        }
    }
}
