/// The [`lexicog`] namespace: https://www.w3.org/ns/lemon/lexicog.
///
/// See [The OntoLex Lemon Lexicography Module](https://www.w3.org/2019/09/lexicog/)
pub mod lexicog {
    use sophia_api::namespace;

    namespace!(
        "http://www.w3.org/ns/lemon/lexicog#",
        describes,
        entry,
        restrictedTo,
        subComponent,
        usageExamples
    );
}

/// The [`ontolex`] namespace: http://www.w3.org/ns/lemon/ontolex.
///
/// See [Lexicon Model for Ontologies: Community Report, 10 May 2016](https://www.w3.org/community/ontolex/wiki/Final_Model_Specification)
pub mod ontolex {
    use sophia_api::namespace;

    namespace!(
        "http://www.w3.org/ns/lemon/ontolex#",
        canonicalForm,
        concept,
        denotes,
        evokes,
        isConceptOf,
        isDenotedBy,
        isEvokedBy,
        isLexicalSenseOf,
        isReferenceOf,
        isSenseOf,
        lexicalForm,
        lexicalizedSense,
        morphologicalPattern,
        otherForm,
        reference,
        sense,
        usage
    );
}

/// The [`synsem`] namespace: http://www.w3.org/ns/lemon/synsem.
pub mod synsem {
    use sophia_api::namespace;

    namespace!(
        "http://www.w3.org/ns/lemon/synsem#",
        condition,
        isA,
        marker,
        objOfProp,
        ontoCorrespondence,
        ontoMapping,
        propertyDomain,
        propertyRange,
        subjOfProp,
        submap,
        synArg,
        synBehavior
    );
}
