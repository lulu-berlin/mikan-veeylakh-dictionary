use serde::Deserialize;
use std::collections::{HashMap, HashSet};

/// The JSON object returned by a SPARQL query.
#[derive(Deserialize, Default, PartialEq, Debug)]
pub struct SparqlResults {
    head: SparqlVars,
    results: SparqlResultsResults,
}

/// The `head` field in the JSON object returned by a SPARQL query.
#[derive(Deserialize, Default, PartialEq, Debug)]
pub struct SparqlVars {
    vars: HashSet<String>,
}

/// The `results` field in the JSON object returned by a SPARQL query.
#[derive(Deserialize, Default, PartialEq, Debug)]
pub struct SparqlResultsResults {
    bindings: Vec<HashMap<String, String>>,
}

impl SparqlResults {
    /// Create an empty SparqlResults object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new SparqlResults object with variables.
    pub fn with_vars<'a, I: IntoIterator<Item = &'a str>>(self, new_vars: I) -> Self {
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
