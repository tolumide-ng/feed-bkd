use validator::{Validate, ValidationError, ValidationErrors};

pub struct FieldValidator {
    errors: ValidationErrors,
}

#[derive(Debug)]
pub struct Errors {
    errors: ValidationErrors,
}

impl Default for FieldValidator {
    fn default() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }
}

impl FieldValidator {
    pub fn validate<T: Validate>(model: &T) -> Self {
        Self {
            errors: model.validate().err().unwrap_or_else(ValidationErrors::new),
        }
    }

    pub fn check(self) -> Result<(), Errors> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(Errors {
                errors: self.errors,
            })
        }
    }

    pub fn extract<T>(&mut self, field_name: &'static str, field: Option<T>) -> T
    where
        T: Default,
    {
        field.unwrap_or_else(|| {
            self.errors
                .add(field_name, ValidationError::new("can't be empty"));
            T::default()
        })
    }
}
