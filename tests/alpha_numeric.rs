extern crate typed_value;

use typed_value::*;

const ALPHA_NUMERIC_REGEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^[A-Za-z0-9]+$").expect("invalid regex"));

enum AlphaNumericValidation {}

impl Validate for AlphaNumericValidation {
    type Value = String;
    type Error = ();

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        if ALPHA_NUMERIC_REGEX.is_match(value) {
            Ok(())
        } else {
            Err(())
        }
    }
}

struct FixedLengthValidation<T, const N: usize>(T);

impl<T: Validate<Value = String, Error = ()>, const N: usize> Validate
    for FixedLengthValidation<T, N>
{
    type Value = String;
    type Error = ();

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        T::validate(value)?;

        if value.chars().count() == N {
            Ok(())
        } else {
            Err(())
        }
    }
}

type DynamicLengthAlphaNumericString = TypedString<AlphaNumericValidation>;

type FixedLengthAlphaNumericString<const N: usize> =
    TypedString<FixedLengthValidation<AlphaNumericValidation, N>>;

#[test]
fn ok_when_initialized_with_alpha_numeric_string() {
    assert!(DynamicLengthAlphaNumericString::new("0123456789ABCDEF".into()).is_ok());
    assert!(FixedLengthAlphaNumericString::<16>::new("0123456789ABCDEF".into()).is_ok());
}

#[test]
fn err_when_initialized_with_alpha_numeric_string() {
    assert!(DynamicLengthAlphaNumericString::new("☺️".into()).is_err());
    assert!(FixedLengthAlphaNumericString::<1>::new("☺".into()).is_err());
}
