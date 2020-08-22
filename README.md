# Mikan Ve'eylakh Dictionary

This is a project of a multilingual dictionary for Hebrew, German and Yiddish.

The project is abbreviated as MVD.

## Structure

The dictionary is stored as an `RDF graph`. It is queried with `SPARQL`. The schema is based on `lemon#ontolex`.

The presentation of the dictionary is based on `gatsby.js` using a `GraphQL` adaptor.

## Components

### Model

Containing the types for the RDF schema and the GraphQL domain model.
