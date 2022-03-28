use serde_derive::Serialize;
use url::Url;

use query_string::QueryString;

const EXAMPLE_URL: &str = "http://example.net/";
#[inline]
fn example_url() -> Url {
    Url::parse(EXAMPLE_URL).unwrap()
}

#[derive(Serialize, Debug)]
struct Unit;

const UNIT_EXPECTED: &str = "";

#[test]
fn new_serializable_unit_struct() {
    let params = Unit;
    let query = QueryString::try_new(&params).unwrap();
    assert_eq!(UNIT_EXPECTED, String::from(query));
}

#[test]
fn append_serializable_unit_struct() {
    let params = Unit;
    let mut query = QueryString::new();
    query.try_append(&params).unwrap();
    assert_eq!(UNIT_EXPECTED, String::from(query));
}

#[test]
fn extend_serializable_unit_struct() {
    let params = [Unit];
    let mut query = QueryString::new();
    query.try_extend(&params).unwrap();
    assert_eq!(UNIT_EXPECTED, String::from(query));
}

#[test]
fn set_url_with_serializable_unit_struct() {
    let params = Unit;
    let query = QueryString::try_new(&params).unwrap();
    let mut url = example_url();
    query.set_url(&mut url);
    assert_eq!(format!("{EXAMPLE_URL}?{UNIT_EXPECTED}"), url.as_str());
}

#[derive(Serialize, Debug, PartialEq, Clone)]
struct SingleNewtype<T>(T);

const SINGLE_NEWTYPE_EXPECTED: &str = "field=newtype";

#[test]
fn new_serializable_newtype_as_value() {
    let params = [("field", SingleNewtype("newtype"))];
    let query = QueryString::try_new(&params).unwrap();
    assert_eq!(SINGLE_NEWTYPE_EXPECTED, String::from(query));
}

#[test]
fn append_serializable_newtype_as_value() {
    let params = [("field", SingleNewtype("newtype"))];
    let mut query = QueryString::new();
    query.try_append(&params).unwrap();
    assert_eq!(SINGLE_NEWTYPE_EXPECTED, String::from(query));
}

#[test]
fn extend_serializable_newtype_as_value() {
    let params = [[("field", SingleNewtype("newtype"))]];
    let mut query = QueryString::new();
    query.try_extend(&params).unwrap();
    assert_eq!(SINGLE_NEWTYPE_EXPECTED, String::from(query));
}

#[test]
fn set_url_with_serializable_newtype_as_value() {
    let params = [("field", SingleNewtype("newtype"))];
    let query = QueryString::try_new(&params).unwrap();
    let mut url = example_url();
    query.set_url(&mut url);
    assert_eq!(format!("{EXAMPLE_URL}?{SINGLE_NEWTYPE_EXPECTED}"), url.as_str());
}

const SINGLE_NEWTYPE_PANIC: &str =
    "SingleNewtype is not supported as top-level input by serde_urlencoded::Serializer";

#[test]
fn new_unserializable_newtype_fails() {
    let params = SingleNewtype("newtype");
    let query = QueryString::try_new(&params);
    match query {
        Ok(_) => panic!("{SINGLE_NEWTYPE_PANIC}"),
        // Fail is expected.
        Err(_) => {}
    }
}

#[test]
fn append_unserializable_newtype_fails() {
    let params = SingleNewtype("newtype");
    let mut query = QueryString::new();
    match query.try_append(&params) {
        Ok(_) => panic!("{SINGLE_NEWTYPE_PANIC}"),
        // Fail is expected.
        Err(_) => {}
    }
}

#[test]
fn extend_unserializable_newtype_fails() {
    let params = [SingleNewtype("newtype")];
    let mut query = QueryString::new();
    let err = match query.try_extend(params.clone()) {
        Ok(_) => panic!("{SINGLE_NEWTYPE_PANIC}"),
        // Fail is expected.
        Err(err) => err,
    };

    assert_eq!(params[0], *err.unserialized());
    assert_eq!(params[0], err.into_unserialized());
}

#[derive(Serialize, Debug)]
struct Single<T> {
    single: T,
}

#[inline]
fn single() -> Single<&'static str> {
    Single { single: "single" }
}

const SINGLE_EXPECTED: &str = "single=single";

#[test]
fn new_serializable_single_field_struct() {
    let params = single();
    let query = QueryString::try_new(&params).unwrap();
    assert_eq!(SINGLE_EXPECTED, String::from(query));
}

#[test]
fn append_serializable_single_field_struct() {
    let params = single();
    let mut query = QueryString::new();
    query.try_append(&params).unwrap();
    assert_eq!(SINGLE_EXPECTED, String::from(query));
}

#[test]
fn extend_serializable_single_field_struct() {
    let params = [single()];
    let mut query = QueryString::new();
    query.try_extend(&params).unwrap();
    assert_eq!(SINGLE_EXPECTED, String::from(query));
}

#[test]
fn set_url_with_serializable_single_field_struct() {
    let params = single();
    let query = QueryString::try_new(&params).unwrap();
    let mut url = example_url();
    query.set_url(&mut url);
    assert_eq!(format!("{EXAMPLE_URL}?{SINGLE_EXPECTED}"), url.as_str());
}

#[derive(Serialize, Debug)]
struct Compound<T> {
    first: T,
    middle: T,
    last: T,
}

const COMPOUND_EXPECTED: &str = "first=first&middle=middle&last=last";

#[inline]
fn compound() -> Compound<&'static str> {
    Compound { first: "first", middle: "middle", last: "last" }
}

#[test]
fn new_serializable_compound_struct() {
    let params = compound();
    let query = QueryString::try_new(&params).unwrap();
    assert_eq!(COMPOUND_EXPECTED, String::from(query));
}

#[test]
fn append_serializable_compound_struct() {
    let params = compound();
    let mut query = QueryString::new();
    query.try_append(&params).unwrap();
    assert_eq!(COMPOUND_EXPECTED, String::from(query));
}

#[test]
fn extend_serializable_compound_struct() {
    let params = [compound()];
    let mut query = QueryString::new();
    query.try_extend(&params).unwrap();
    assert_eq!(COMPOUND_EXPECTED, String::from(query));
}

#[test]
fn set_url_with_serializable_compound_struct() {
    let params = compound();
    let query = QueryString::try_new(&params).unwrap();
    let mut url = example_url();
    query.set_url(&mut url);
    assert_eq!(format!("{EXAMPLE_URL}?{COMPOUND_EXPECTED}"), url.as_str());
}
