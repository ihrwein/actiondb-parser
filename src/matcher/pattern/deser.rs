use super::Pattern;
use super::testmessage::TestMessage;

use serde::de::Deserialize;
use serde;
use uuid::Uuid;

use std::collections::BTreeMap;

impl serde::Deserialize for Pattern {
    fn deserialize<D>(deserializer: &mut D) -> Result<Pattern, D::Error>
        where D: serde::de::Deserializer
    {
        deserializer.visit_struct("Pattern", &[], PatternVisitor)
    }
}

enum Field {
    NAME,
    UUID,
    PATTERN,
    VALUES,
    TAGS,
    TESTMESSAGES,
}

impl serde::Deserialize for Field {
    fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
        where D: serde::de::Deserializer
    {
        struct FieldVisitor;

        impl serde::de::Visitor for FieldVisitor {
            type Value = Field;

            fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                where E: serde::de::Error
            {
                match value {
                    "name" => Ok(Field::NAME),
                    "uuid" => Ok(Field::UUID),
                    "pattern" => Ok(Field::PATTERN),
                    "values" => Ok(Field::VALUES),
                    "tags" => Ok(Field::TAGS),
                    "test_messages" => Ok(Field::TESTMESSAGES),
                    _ => Err(serde::de::Error::syntax(&format!("Unexpected field: {}", value))),
                }
            }
        }

        deserializer.visit(FieldVisitor)
    }
}


struct PatternVisitor;

impl PatternVisitor {
    pub fn parse_uuid<V: serde::de::MapVisitor>(uuid: Option<String>) -> Result<Uuid, V::Error> {
        let uuid = match uuid {
            Some(uuid) => {
                match Uuid::parse_str(&uuid) {
                    Ok(value) => Some(value),
                    Err(err) => {
                        error!("Invalid field 'uuid': uuid={:?} error={}", &uuid, err);
                        None
                    }
                }
            }
            None => {
                None
            }
        };

        match uuid {
            Some(uuid) => Ok(uuid),
            None => {
                try!(Err(serde::de::Error::missing_field("uuid")))
            }
        }
    }
}

impl serde::de::Visitor for PatternVisitor {
    type Value = Pattern;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<Pattern, V::Error>
        where V: serde::de::MapVisitor
    {
        let mut name = None;
        let mut uuid: Option<String> = None;
        let mut pattern: Option<String> = None;
        let mut values: Option<BTreeMap<String, String>> = None;
        let mut tags: Option<Vec<String>> = None;
        let mut test_messages: Option<Vec<TestMessage>> = None;

        loop {
            match try!(visitor.visit_key()) {
                Some(Field::NAME) => {
                    name = Some(try!(visitor.visit_value()));
                }
                Some(Field::UUID) => {
                    uuid = Some(try!(visitor.visit_value()));
                }
                Some(Field::PATTERN) => {
                    pattern = Some(try!(visitor.visit_value()));
                }
                Some(Field::VALUES) => {
                    values = Some(try!(visitor.visit_value()));
                }
                Some(Field::TAGS) => {
                    tags = Some(try!(visitor.visit_value()));
                }
                Some(Field::TESTMESSAGES) => {
                    test_messages = Some(try!(visitor.visit_value()));
                }
                None => {
                    break;
                }
            }
        }

        let uuid = try!(PatternVisitor::parse_uuid::<V>(uuid));
        let name = match name {
            Some(name) => name,
            None => try!(visitor.missing_field("name")),
        };

        let pattern = match pattern {
            Some(pattern) => {
                match ::grammar::parser::pattern(&pattern) {
                    Ok(pattern) => pattern,
                    Err(err) => {
                        error!("Invalid field 'pattern': pattern={:?} name={:?} uuid={:?} \
                                error={}",
                               pattern,
                               name,
                               uuid,
                               err);
                        try!(Err(serde::de::Error::syntax("Invalid field 'pattern'")))
                    }
                }
            }
            None => {
                error!("Missing field 'pattern': name={:?} uuid={:?}", name, uuid);
                try!(Err(serde::de::Error::missing_field("pattern")))
            }
        };


        try!(visitor.end());

        Ok(Pattern::new(name, uuid, pattern, test_messages, values, tags))
    }
}
