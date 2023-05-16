use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buff> {
    data: HashMap<&'buff str, Value<'buff>>,
}

#[derive(Debug)]
pub enum Value<'buff> {
    Single(&'buff str),
    Multiple(Vec<&'buff str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split("&") {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find("=") {
                key = &sub_str[..i];
                val = &sub_str[i + 1..]
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        //This the first way
                        *existing = Value::Multiple(vec![prev_val, val]);

                        // //Second Method
                        // let mut vec = Vec::new()
                        // vec.push(key);
                        // vec.push(prev_val);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data }
        // unimplemented!()
    }
}
