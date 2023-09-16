use actix_web::HttpRequest;
use crate::cond_builder::CondBuilder;

const LIMIT: u32 = 15;

#[derive(Debug)]
#[allow(dead_code)]
pub struct QueryBuilder<'a> { 
    pub table_name: &'a str,
    pub fields: &'a str,
    pub order: String,
    pub page_size: u32,
    pub offset: u32,
    pub page: u32,
    query: String,
}

impl<'a> QueryBuilder<'a> { 

    pub fn new() -> Self { 
        QueryBuilder{ 
            table_name: "",
            order: "".to_owned(),
            fields: "*",
            page_size: LIMIT,
            offset: 0,
            page: 1,
            query: "".into(),
        }
    }
    
    pub fn from(&mut self, table_name: &'a str) -> &mut Self { 
        self.table_name = table_name;
        self
    }

    pub fn fields(&mut self, fields: &'a str) -> &mut Self { 
        self.fields = fields;
        self
    }

    pub fn limit(&mut self, limit: u32) -> &mut Self { 
        self.page_size = limit;
        self
    }

    pub fn offset(&mut self, offset: u32) -> &mut Self { 
        self.offset = offset;
        self
    }

    pub fn order_by(&mut self, order: &'a str) -> &mut Self { 
        self.order = format!("ORDER BY {}", order);
        self
    }

    pub fn build_query(&self, table_name: &str, cond: Option<&CondBuilder>)-> String { 
        format!("SELECT {} FROM {} {} {} LIMIT {}, {}", 
            self.fields, 
            if table_name != "" { table_name } else { self.table_name },
            if let Some(v) = cond { format!("WHERE {}", v.build()) } else { "".to_owned() },
            self.order,
            self.offset, 
            self.page_size)
    }

    pub fn build_query_first(&self, table_name: &str, cond: Option<&CondBuilder>)-> String { 
        format!("SELECT {} FROM {} {} {} LIMIT 1", 
            self.fields, 
            if table_name != "" { table_name } else { self.table_name },
            if let Some(v) = cond { format!("WHERE {}", v.build()) } else { "".to_owned() },
            self.order)
    }
    
    pub fn build_query_total(&self, table_name: &str, cond: Option<&CondBuilder>) -> String { 
        format!("SELECT COUNT(*) AS total  FROM {} {}", 
            if table_name != "" { table_name } else { self.table_name },
            if let Some(v) = cond { format!("WHERE {}", v.build()) } else { "".to_owned() })
    }
    
    /// 设置limit和offset
    /// 參數: page  => 頁數
    /// 参数: limit => 每頁記錄數
    pub fn set_limit_offset(&mut self, req: &HttpRequest) { 
        let query_string = req.query_string();
        let queries = query_string.split("&").collect::<Vec<&str>>();
        if queries.len() == 0 { 
            return;
        }
        for q in queries { 
            let query_arr = q.split("=").collect::<Vec<&str>>();
            if query_arr.len() != 2 { 
                continue;
            }
            let key = query_arr[0];
            let val = query_arr[1];
            if key == "page" { 
                if let Ok(v) = val.parse::<u32>() { 
                    self.page = v;
                }
            } else if key == "limit" { 
                if let Ok(v) = val.parse::<u32>() { 
                    self.page_size = v;
                };
            }
        }

        self.offset = (self.page - 1) * self.page_size;
    }
}

#[macro_export]
macro_rules! query {
    [$($key: ident => $val: expr,)*] => ({
        let mut builder = fluffy::query_builder::QueryBuilder::new();
        $(builder.$key($val);)*
        builder
    })
}   
