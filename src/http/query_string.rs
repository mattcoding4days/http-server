use std::collections::HashMap;

/* Here we can see that in the case of the key 'd',
 * we must store an array of values, but for all other keys,
 * we only need a simple string slice => &str
 * */
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Many(Vec<&'buf str>),
}

#[derive(Debug)]
pub struct QueryString<'buf> {
    m_data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.m_data.get(key)
    }
}

// Example query string: a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data: HashMap<&'buf str, Value<'buf>> = HashMap::new();

        for sub_str in s.split('&') {
            // first we explode the string on '&'
            let mut key = sub_str;
            let mut val = "";
            // check if there is an '=' sign
            if let Some(i) = sub_str.find('=') {
                // if there is, the key will now be equal to
                // everything before the '=', and val will take
                // everything after the equal sign
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Many(vec![prev_val, val]);
                    }
                    Value::Many(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }
        QueryString { m_data: data }
    }
}
