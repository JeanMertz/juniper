use std::collections::HashSet;

use super::schema_introspection::*;
use crate::executor::Variables;
use crate::introspection::IntrospectionFormat;
use crate::schema::model::RootNode;
use crate::tests::model::Database;
use crate::tests::schema::Query;
use crate::types::scalars::EmptyMutation;

#[test]
fn test_introspection_query_type_name() {
    let doc = r#"
        query IntrospectionQueryTypeQuery {
          __schema {
            queryType {
              name
            }
          }
        }"#;
    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    assert_eq!(
        crate::execute(doc, None, &schema, &Variables::new(), &database),
        Ok((
            graphql_value!({
                "__schema": {
                    "queryType": {
                        "name": "Query"
                    }
                }

            }),
            vec![]
        ))
    );
}

#[test]
fn test_introspection_type_name() {
    let doc = r#"
        query IntrospectionQueryTypeQuery {
          __type(name: "Droid") {
            name
          }
        }"#;
    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    assert_eq!(
        crate::execute(doc, None, &schema, &Variables::new(), &database),
        Ok((
            graphql_value!({
                "__type": {
                    "name": "Droid",
                },
            }),
            vec![]
        ))
    );
}

#[test]
fn test_introspection_specific_object_type_name_and_kind() {
    let doc = r#"
        query IntrospectionDroidKindQuery {
          __type(name: "Droid") {
            name
            kind
          }
        }
        "#;
    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    assert_eq!(
        crate::execute(doc, None, &schema, &Variables::new(), &database),
        Ok((
            graphql_value!({
                "__type": {
                    "name": "Droid",
                    "kind": "OBJECT",
                }
            }),
            vec![],
        ))
    );
}

#[test]
fn test_introspection_specific_interface_type_name_and_kind() {
    let doc = r#"
        query IntrospectionDroidKindQuery {
          __type(name: "Character") {
            name
            kind
          }
        }
        "#;
    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    assert_eq!(
        crate::execute(doc, None, &schema, &Variables::new(), &database),
        Ok((
            graphql_value!({
                "__type": {
                    "name": "Character",
                    "kind": "INTERFACE",
                }
            }),
            vec![]
        ))
    );
}

#[test]
fn test_introspection_documentation() {
    let doc = r#"
        query IntrospectionDroidDescriptionQuery {
          __type(name: "Droid") {
            name
            description
          }
        }
        "#;
    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    assert_eq!(
        crate::execute(doc, None, &schema, &Variables::new(), &database),
        Ok((
            graphql_value!({
                "__type": {
                    "name": "Droid",
                    "description": "A mechanical creature in the Star Wars universe.",
                },
            }),
            vec![]
        ))
    );
}

#[test]
fn test_introspection_directives() {
    let q = r#"
        query IntrospectionQuery {
          __schema {
            directives {
              name
              locations
            }
          }
        }
    "#;

    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    let mut result = crate::execute(q, None, &schema, &Variables::new(), &database).unwrap();
    sort_schema_value(&mut result.0);

    let mut expected = graphql_value!({
        "__schema": {
            "directives": [
                {
                    "name": "include",
                    "locations": [
                        "FIELD",
                        "FRAGMENT_SPREAD",
                        "INLINE_FRAGMENT",
                    ],
                },
                {
                    "name": "skip",
                    "locations": [
                        "FIELD",
                        "FRAGMENT_SPREAD",
                        "INLINE_FRAGMENT",
                    ],
                },
            ],
        },
    });
    sort_schema_value(&mut expected);

    assert_eq!(result, (expected, vec![]));
}

#[test]
fn test_introspection_possible_types() {
    let doc = r#"
        query IntrospectionDroidDescriptionQuery {
          __type(name: "Character") {
            possibleTypes {
              name
            }
          }
        }
        "#;
    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    let result = crate::execute(doc, None, &schema, &Variables::new(), &database);

    let (result, errors) = result.ok().expect("Query returned error");

    assert_eq!(errors, vec![]);

    let possible_types = result
        .as_object_value()
        .expect("execution result not an object")
        .get_field_value("__type")
        .expect("'__type' not present in result")
        .as_object_value()
        .expect("'__type' not an object")
        .get_field_value("possibleTypes")
        .expect("'possibleTypes' not present in '__type'")
        .as_list_value()
        .expect("'possibleTypes' not a list")
        .iter()
        .map(|t| {
            t.as_object_value()
                .expect("possible type not an object")
                .get_field_value("name")
                .expect("'name' not present in type")
                .as_scalar_value::<String>()
                .expect("'name' not a string") as &str
        })
        .collect::<HashSet<_>>();

    assert_eq!(possible_types, vec!["Human", "Droid"].into_iter().collect());
}

#[test]
fn test_builtin_introspection_query() {
    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    let mut result = crate::introspect(&schema, &database, IntrospectionFormat::default()).unwrap();
    sort_schema_value(&mut result.0);
    let expected = schema_introspection_result();
    assert_eq!(result, (expected, vec![]));
}

#[test]
fn test_builtin_introspection_query_without_descriptions() {
    let database = Database::new();
    let schema = RootNode::new(Query, EmptyMutation::<Database>::new());

    let mut result =
        crate::introspect(&schema, &database, IntrospectionFormat::WithoutDescriptions).unwrap();
    sort_schema_value(&mut result.0);
    let expected = schema_introspection_result_without_descriptions();

    assert_eq!(result, (expected, vec![]));
}
