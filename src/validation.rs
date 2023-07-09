use std::default::Default;

#[derive(Default)]
pub struct Validator {
    errors: Vec<String>,
}

impl Validator {

    /// Generate a mandarin
    pub fn new() -> Self { 
        Self { 
            errors: vec![],
        }       
    }

    /// user name
    /// It takes 6-15 bits, A-ZA-Z0-9, and cannot start with numbers
    pub fn is_username<T: AsRef<str>>(&mut self, value: &T, message: &str) -> &mut Self {
        let length = value.as_ref().chars().count();
        if length < 6 || length > 15 {
            self.errors.push(message.to_owned());
        }
        self
    }

    /// Whether password
    pub fn is_password<T: AsRef<str>>(&mut self, value: &T, message: &str) -> &mut Self {
        let length = value.as_ref().chars().count();
        if length < 6 || length > 20 { 
            self.errors.push(message.to_owned());
        }
        // TODO: need to check the illegal letters
        self
    }

    /// Is it a duplicate password
    pub fn is_re_password<T: AsRef<str>>(&mut self, value: &T, re_value: &T, message: &str) -> &mut Self {
        let length = value.as_ref().chars().count();
        if length < 6 || length > 20 { 
            self.errors.push(message.to_owned());
        }
        let length2 = re_value.as_ref().chars().count();
        if length2 < 6 ||  length2 > 20 { 
            self.errors.push(message.to_owned());
        }
        if value.as_ref() != re_value.as_ref() { 
            self.errors.push(message.to_owned());
        }
        // TODO: need to check the illegal letters
        self
    }

    /// Whether it is a nickname
    pub fn is_nickname<T: AsRef<str>>(&mut self, value: &T, message: &str) -> &mut Self { 
        let length = value.as_ref().chars().count();
        if length < 3 || length > 12 {
            self.errors.push(message.to_owned());
        }
        self
    }

    /// Whether it is the verification code
    pub fn is_code(&mut self, value: u32, message: &str) -> &mut Self { 
        if value < 10000 || value > 99999 { 
            self.errors.push(message.to_owned());
        }
        self
    }

    /// is status
    pub fn is_state(&mut self, value: u32, message: &str) -> &mut Self { 
        if value != 0 && value != 1 { 
            self.errors.push(message.to_owned());
        }
        self
    }

    /// in range ?
    pub fn in_range(&mut self, min: u32, max: u32, value: u32, message: &str) -> &mut Self { 
        if min < value || max > value { 
            self.errors.push(message.to_owned());
        }
        self
    }

    /// get validated result
    pub fn validate(&mut self) -> Result<(), String> {
        if self.errors.len() > 0 {
            return Err(self.errors.join(","));
        }
        Ok(())
    }
}
