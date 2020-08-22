mod support;
use support::{oxigraph_docker::OxigraphContainer, sparql::SparqlDBClient, sparql::SparqlResults};

#[test]
fn start_oxigraph_docker() -> Result<(), Box<dyn std::error::Error>> {
    let oxigraph = OxigraphContainer::spawn();

    assert_eq!(
        oxigraph.sparql_query(
            r"SELECT *
            WHERE { ?s ?p ?o }
            LIMIT 10"
        )?,
        SparqlResults::new().with_vars(vec!["s", "p", "o"])
    );

    Ok(())
}
