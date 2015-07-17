use std::collections::BTreeMap;
use std::borrow::Borrow;

#[cfg(test)]
mod test;
mod deser;

#[derive(Clone, Debug)]
pub struct TestMessage {
    message: String,
    values: BTreeMap<String, String>
}

impl TestMessage {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    pub fn test_pairs(&self, pairs: &[(&str, &str)]) -> Result<(), TestPairsError> {
        if pairs.len() != self.values().len() {
            Err(TestPairsError::invalid_length(self.values.len(), pairs.len()))
        } else  {
            self.test_pairs_values(pairs)
        }
    }

    pub fn test_pairs_values(& self, pairs: &[(&str, &str)]) -> Result<(), TestPairsError> {
        for &(key, value) in pairs {
            let expected_value = self.values().get(key).map(|x| x.borrow());
            if let Some(exp) = expected_value {
                if exp != value {
                    return Err(TestPairsError::value_not_match(key, exp, value));
                }
            } else {
                return Err(TestPairsError::key_not_found(key));
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TestPairsError {
    InvalidLength{expected: usize, got: usize},
    ValueNotMatch{key: String, expected_value: String, got_value: String},
    KeyNotFound{key: String}
}

impl TestPairsError {
    pub fn invalid_length(expected: usize, got: usize) -> TestPairsError {
        TestPairsError::InvalidLength{expected: expected, got: got}
    }

    pub fn value_not_match(key: &str, expected_value: &str, got_value: &str) -> TestPairsError {
        TestPairsError::ValueNotMatch{key: key.to_string(), expected_value: expected_value.to_string(), got_value: got_value.to_string()}
    }

    pub fn key_not_found(key: &str) -> TestPairsError {
        TestPairsError::KeyNotFound{key: key.to_string()}
    }
}
