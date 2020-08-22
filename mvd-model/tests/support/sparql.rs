use serde::Deserialize;
use std::collections::{HashMap, HashSet};

/// A database that can be spawned and accept SPARQL queries.
pub trait SparqlDBClient {
    fn spawn() -> Self;
    fn sparql_query(&self, query: &str) -> Result<SparqlResults, Box<dyn std::error::Error>>;
}

/// The JSON object returned by a SPARQL query.
#[derive(Deserialize, Default, PartialEq, Debug)]
pub(crate) struct SparqlResults {
    head: SparqlVars,
    results: SparqlResultsResults,
}

/// The `head` field in the JSON object returned by a SPARQL query.
#[derive(Deserialize, Default, PartialEq, Debug)]
pub(crate) struct SparqlVars {
    vars: HashSet<String>,
}

/// The `results` field in the JSON object returned by a SPARQL query.
#[derive(Deserialize, Default, PartialEq, Debug)]
pub(crate) struct SparqlResultsResults {
    bindings: Vec<HashMap<String, String>>,
}

impl SparqlResults {
    /// Create an empty SparqlResults object.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Create a new SparqlResults object with variables.
    pub(crate) fn with_vars<'a, I: IntoIterator<Item = &'a str>>(self, new_vars: I) -> Self {
        let Self {
            head: SparqlVars { mut vars },
            results,
        } = self;
        vars.extend(new_vars.into_iter().map(String::from));
        Self {
            head: SparqlVars { vars },
            results,
        }
    }
}
