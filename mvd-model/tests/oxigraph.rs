mod support;
use oxigraph::sparql::Variable;
use support::{
    oxigraph_docker::OxigraphContainer, oxigraph_memory_store::OxigraphMemoryStore,
    sparql::SparqlDBClient,
};

#[test]
fn start_oxigraph_docker() -> Result<(), Box<dyn std::error::Error>> {
    let oxigraph = OxigraphContainer::spawn();

    let turtle = include_str!("fixtures/example1.ttl");

    oxigraph.insert_turtle(turtle)?;

    let solutions = oxigraph.sparql_query(
        r"SELECT *
            WHERE { ?s ?p ?o }",
    )?;

    let variables = solutions.variables();

    assert_eq!(variables.len(), 3);
    assert!(variables.iter().any(|v| *v == Variable::new("s")));
    assert!(variables.iter().any(|v| *v == Variable::new("p")));
    assert!(variables.iter().any(|v| *v == Variable::new("o")));

    let triples: Vec<(String, String, String)> = solutions
        .map(|solution| {
            let solution = solution.unwrap();
            (
                solution.get("s").unwrap().to_string(),
                solution.get("p").unwrap().to_string(),
                solution.get("o").unwrap().to_string(),
            )
        })
        .collect();

    assert!(triples
        .iter()
        .any(|(s, p, o)| s == "<http://example.org/#spiderman>"
            && p == "<http://xmlns.com/foaf/0.1/name>"
            && o == "\"Spiderman\""));

    assert!(triples
        .iter()
        .any(|(s, p, o)| s == "<http://example.org/#green-goblin>"
            && p == "<http://xmlns.com/foaf/0.1/name>"
            && o == "\"Green Goblin\""));

    Ok(())
}

#[test]
fn start_oxigraph_memory_store() -> Result<(), Box<dyn std::error::Error>> {
    let oxigraph = OxigraphMemoryStore::spawn();

    let turtle = include_str!("fixtures/example1.ttl");

    oxigraph.insert_turtle(turtle)?;

    let solutions = oxigraph.sparql_query(
        r"SELECT *
            WHERE { ?s ?p ?o }",
    )?;

    let variables = solutions.variables();

    assert_eq!(variables.len(), 3);
    assert!(variables.iter().any(|v| *v == Variable::new("s")));
    assert!(variables.iter().any(|v| *v == Variable::new("p")));
    assert!(variables.iter().any(|v| *v == Variable::new("o")));

    let triples: Vec<(String, String, String)> = solutions
        .map(|solution| {
            let solution = solution.unwrap();
            (
                solution.get("s").unwrap().to_string(),
                solution.get("p").unwrap().to_string(),
                solution.get("o").unwrap().to_string(),
            )
        })
        .collect();

    assert!(triples
        .iter()
        .any(|(s, p, o)| s == "<http://example.org/#spiderman>"
            && p == "<http://xmlns.com/foaf/0.1/name>"
            && o == "\"Spiderman\""));

    assert!(triples
        .iter()
        .any(|(s, p, o)| s == "<http://example.org/#green-goblin>"
            && p == "<http://xmlns.com/foaf/0.1/name>"
            && o == "\"Green Goblin\""));

    Ok(())
}
