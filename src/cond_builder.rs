use std::fmt::Display;

/// Generate query conditions
///
/// Call method: 
///
/// ```rust
/// let _cond = CondBuilder::new()
///     .eq("id", 1)
///     .gt("age", 30)
///     .ne("name", "james")
///     .lt("score", 90)
///     .build();
/// // As follows:
/// let _cond = cond![
///     "id" => &1
/// ];
/// ```
#[derive(Debug)]
pub struct CondBuilder { 
    conditions: String,
    count: usize,
}

impl<'a> CondBuilder { 
    
    /// Create new CondBuilder
    pub fn new() -> Self { 
        CondBuilder{ 
            conditions: "1 = 1".into(),
            count: 0,
        }
    }
    
    /// append new condition
    pub fn append(&mut self, where_str: &str) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {}", where_str));
        self.count += 1;
        self
    }

    /// equal
    pub fn eq<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} = '{}'", key, value));
        self.count += 1;
        self
    }

    /// not equal to
    pub fn ne<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} <> '{}'", key, value));
        self.count += 1;
        self
    }

    /// more than the
    pub fn gt<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} > '{}'", key, value));
        self.count += 1;
        self
    }

    /// greater or equal to
    pub fn gte<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} >= '{}'", key, value));
        self.count += 1;
        self
    }

    /// Less than
    pub fn lt<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} < '{}'", key, value));
        self.count += 1;
        self
    }

    /// Less than equal
    pub fn lte<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} <= '{}'", key, value));
        self.count += 1;
        self
    }

    /// LIKE
    pub fn like<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} LIKE '%{}%'", key, value));
        self.count += 1;
        self
    }

    /// BETWEEN
    pub fn between<T: Display>(&mut self, key: &str, value1: &T, value2: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} BETWEEN '{}' AND '{}'", key, value1, value2));
        self.count += 1;
        self
    }

    /// OR
    pub fn or(&mut self, cond: &CondBuilder) -> &mut Self { 
        self.conditions.push_str(&format!(" OR ({})", cond.build()));
        self.count += 1;
        self
    }

    /// IN
    pub fn in_range<T: Display>(&mut self, key: &str, value: &T) -> &mut Self { 
        self.conditions.push_str(&format!(" AND {} IN ({})", key, value));
        self.count += 1;
        self
    }
    
    /// CLEAR
    pub fn clear(&mut self) -> &mut Self { 
        self.conditions = "1 = 1".to_owned();
        self
    }

    /// build
    pub fn build(&'a self) -> &'a String { 
        &self.conditions
    }
    
    /// Quantity
    pub fn len(&self) -> usize { 
        self.count
    }
}


/// All query conditions identified by the equal number
#[macro_export]
macro_rules! cond { 
    [$($method: ident => [ $($field: expr => &$val: expr,)+],)* ] => ({
        let mut cond = fluffy::cond_builder::CondBuilder::new();
        $($(cond.$method($field, &$val);)*)*
        cond
    });
    [$($key: expr => $val: expr,)*] => ({
        let mut cond = fluffy::cond_builder::CondBuilder::new();
        $(cond.eq($key, &$val);)*
        cond
    });
}
