use std::fmt::{Display, Formatter};

pub struct DataSet<'a> { 
    pub is_create: bool, // create or update
    pub fields: Vec<&'a str>, // fields
    pub values: Vec<String>, // values
}

impl<'a> DataSet<'a> { 

    /// Generate data
    pub fn create() -> Self { 
        Self { 
            is_create: true,
            fields: vec![],
            values: vec![],
        }
    }
    
    /// update data
    pub fn update() -> Self { 
        Self { 
            is_create: false,
            fields: vec![],
            values: vec![],
        }
    }

    /// Empty
    pub fn clear(&mut self) -> &mut Self { 
        self.is_create = true;
        self.fields = vec![];
        self.values = vec![];
        self
    }

    /// Set the field value
    pub fn set<T: ToString>(&mut self, field: &'a str, value: &T) -> &mut Self { 
        if field == "id" { 
            self.is_create = true;
        }
        self.fields.push(field);
        let value_string = value.to_string();
        let real_value = value_string.replace("'", "\'");
        self.values.push(real_value);
        self
    }

    /// Generate SQL statement
    pub fn build(&self) -> String { 
        if self.is_create { 
            return self.build_create();
        }
       self.build_update()
    }

    /// Creating sql
    fn build_create(&self) -> String { 
        let mut sql = String::from("(");
        sql.push_str(self.fields.join(",").as_str());
        sql.push_str(") VALUES ('");
        sql.push_str(self.values.join("','").as_str());
        sql.push_str("')");
        sql
    }

    /// updating sql
    fn build_update(&self) -> String { 
        let length = self.fields.len();
        let mut updates: Vec<String> = vec![];
        for i in 0..length { 
            updates.push(format!("{} = '{}'", &self.fields[i], &self.values[i]));
        }
        updates.join(",")
    }
}

impl<'a> Display for DataSet<'a> { 

    /// Format output data
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { 
        let mut result = String::new();
        let method = if self.is_create { "create" } else { "update" };
        result.push_str(&format!("(method: {}){}\n", method, "{"));
        for i in 0..self.fields.len() { 
            result.push_str(&format!("    {} => {},\n", self.fields[i], self.values[i]));
        }
        result.push_str(&format!("{}", "}"));
        write!(f, "{}", result)
    }
}


/// Generate add data
#[macro_export]
macro_rules! create_row { 
    [$($key: expr => $val: expr,)+] => (
        {
            let mut data = fluffy::data_set::DataSet::create();
            $(data.set($key, &$val);)+
            data
        }
    )
}

/// Generate modification data
#[macro_export]
macro_rules! update_row { 
    [$($key: expr => $val: expr,)*] => (
        {
            let mut data = fluffy::data_set::DataSet::update();
            $(data.set($key, &$val);)*
            data
        }
    )
}
