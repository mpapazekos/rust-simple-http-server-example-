use crate::utils::MapValueType;

use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpHeaderMap<'buf> {
    data: HashMap<&'buf str, MapValueType<'buf>>,
}

//==================================================

impl<'buf> Default for HttpHeaderMap<'buf> {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
        }
    }
}

impl<'buf> HttpHeaderMap<'buf> {
    pub fn get(&self, key: &str) -> Option<&MapValueType> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for HttpHeaderMap<'buf> {
    fn from(header_str: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in header_str.split("\r\n") {
            let mut key = sub_str;

            if let Some(idx) = sub_str.find(':') {
                key = &sub_str[..idx];

                let value_string = &sub_str[idx + 1..];
                let value_iter = value_string.split(',');

                for value in value_iter {
                    if !value.is_empty() {
                        data.entry(key)
                            .and_modify(|existing: &mut MapValueType| match existing {
                                MapValueType::Single(prev_value) => {
                                    *existing = MapValueType::Multiple(vec![prev_value, value])
                                }

                                MapValueType::Multiple(vec) => vec.push(value),
                            })
                            .or_insert(MapValueType::Single(value));
                    }
                }
            }
        }

        return Self { data };
    }
}
