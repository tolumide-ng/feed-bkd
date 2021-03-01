use validator::ValidationError;

pub fn validate_country(country: &str) -> Result<(), ValidationError> {
    if country.to_uppercase().as_str() == "NGN" {
        return Err(ValidationError::new(
            "We currenlty do not offer this service in Nigeria",
        ));
    }

    Ok(())
}
