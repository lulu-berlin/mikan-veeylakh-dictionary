use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use testcontainers::*;

#[derive(Deserialize, Default, PartialEq, Debug)]
struct SparqlResults {
    head: SparqlVars,
    results: SparqlResultsResults,
}

#[derive(Deserialize, Default, PartialEq, Debug)]
struct SparqlVars {
    vars: HashSet<String>,
}

#[derive(Deserialize, Default, PartialEq, Debug)]
struct SparqlResultsResults {
    bindings: Vec<HashMap<String, String>>,
}

impl SparqlResults {
    pub fn new() -> Self {
        Self::default()
    }

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

#[test]
fn start_oxigraph() -> Result<(), Box<dyn std::error::Error>> {
    let oxigraph_image = testcontainers::images::generic::GenericImage::new("oxigraph/oxigraph")
        .with_mapped_port((7878, 7878))
        .with_args(vec![
            "-b".to_string(),
            "0.0.0.0:7878".to_string(),
            "-f".to_string(),
            "/data".to_string(),
        ])
        .with_wait_for(testcontainers::images::generic::WaitFor::message_on_stdout(
            "Listening for requests",
        ));
    let docker = testcontainers::clients::Cli::default();
    let node = docker.run(oxigraph_image);
    let host_port = node.get_host_port(7878).unwrap();
    let url = format!("http://localhost:{}/query", host_port);

    let client = reqwest::blocking::Client::new();
    let resp = match client
        .post(&url)
        .header("Accept", "application/sparql-results+json")
        .header("Content-Type", "application/sparql-query")
        .body(
            r"SELECT *
                WHERE { ?s ?p ?o }
                LIMIT 10
            ",
        )
    .send()
    .and_then(|response| response.json::<SparqlResults>())
    // .and_then(|response| response.text())
    {
        Ok(json) => json,
        Err(err) => {
            docker.stop(node.id());
            panic!("Something went wrong! - {:?}", err)
        }
    };

    docker.stop(node.id());

    assert_eq!(resp, SparqlResults::new().with_vars(vec!["s", "p", "o"]));
    // assert_eq!(resp, "");

    Ok(())
}
