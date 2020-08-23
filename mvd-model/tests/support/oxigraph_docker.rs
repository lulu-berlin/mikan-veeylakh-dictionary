use super::sparql::SparqlResults;
use lazy_static::lazy_static;
use testcontainers::*;

lazy_static! {
    // The docker client instance is borrowed when running a container,
    // so it's easier to make it static.
    static ref DOCKER_CLIENT: testcontainers::clients::Cli =
        testcontainers::clients::Cli::default();
}

/// A wrapper around a GenericImage container.
pub struct OxigraphContainer(
    testcontainers::Container<
        'static,
        testcontainers::clients::Cli,
        testcontainers::images::generic::GenericImage,
    >,
);

impl Drop for OxigraphContainer {
    /// Stop the docker container when it goes out of scope.
    fn drop(&mut self) {
        DOCKER_CLIENT.stop(self.0.id());
    }
}

impl OxigraphContainer {
    /// Spawn a docker container with oxigraph server (with `RocksDB` key-value store).
    pub fn spawn() -> Self {
        // See the README of `oxigraph`
        let image = testcontainers::images::generic::GenericImage::new("oxigraph/oxigraph")
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
        let container = DOCKER_CLIENT.run(image);
        Self(container)
    }

    /// Make a SPARQL query.
    pub fn sparql_query<'a>(
        &self,
        query: &str,
    ) -> Result<SparqlResults, Box<dyn std::error::Error>> {
        let host_port = self.0.get_host_port(7878).unwrap();
        let url = format!("http://localhost:{}/query", host_port);
        let client = reqwest::blocking::Client::new();
        // Based on the example in the README of `oxigraph`:
        //
        // curl -X POST \
        //      -H 'Accept: application/sparql-results+json' \
        //      -H 'Content-Type: application/sparql-query' \
        //      --data 'SELECT * WHERE { ?s ?p ?o } LIMIT 10' \
        //      http://localhost:7878/query
        let response = client
            .post(&url)
            .header("Accept", "application/sparql-results+json")
            .header("Content-Type", "application/sparql-query")
            .body(query.to_string())
            .send()?
            .json::<SparqlResults>()?;
        Ok(response)
    }
}
