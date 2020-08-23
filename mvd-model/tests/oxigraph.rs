mod support;
use oxigraph::sparql::Variable;
use support::{
    oxigraph_docker::OxigraphContainer, oxigraph_memory_store::OxigraphMemoryStore,
    sparql::SparqlDBClient,
};

#[test]
fn start_oxigraph_docker() -> Result<(), Box<dyn std::error::Error>> {
    let oxigraph = OxigraphContainer::spawn();

    let solutions = oxigraph.sparql_query(
        r"SELECT *
            WHERE { ?s ?p ?o }
            LIMIT 10",
    )?;

    let variables = solutions.variables();

    assert_eq!(variables.len(), 3);
    assert!(variables.iter().any(|v| *v == Variable::new("s")));
    assert!(variables.iter().any(|v| *v == Variable::new("p")));
    assert!(variables.iter().any(|v| *v == Variable::new("o")));

    Ok(())
}

#[test]
fn start_oxigraph_memory_store() -> Result<(), Box<dyn std::error::Error>> {
    let oxigraph = OxigraphMemoryStore::spawn();

    let solutions = oxigraph.sparql_query(
        r"SELECT *
            WHERE { ?s ?p ?o }
            LIMIT 10",
    )?;

    let variables = solutions.variables();

    assert_eq!(variables.len(), 3);
    assert!(variables.iter().any(|v| *v == Variable::new("s")));
    assert!(variables.iter().any(|v| *v == Variable::new("p")));
    assert!(variables.iter().any(|v| *v == Variable::new("o")));

    Ok(())
}
