use crate::utils::MapValueType;

use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {

    data: HashMap<&'buf str, MapValueType<'buf>>
}

//==================================================

impl<'buf> QueryString<'buf> {

    pub fn get(&self, key: &str) -> Option<&MapValueType> { self.data.get(key) }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {   

    fn from(qr_str: &'buf str) -> Self { 

        let mut data = HashMap::new();

        for sub_str in qr_str.split('&') {

            let mut key = sub_str;
            let mut value = "";
           
            if let Some(idx) = sub_str.find('=') {

                key = &sub_str[..idx];
                value = &sub_str[idx+1..]
            }

            data.entry(key)
                .and_modify(|existing: &mut MapValueType| 
                    match existing {

                        MapValueType::Single(prev_value) 
                        => *existing = MapValueType::Multiple(vec![prev_value, value]),
                        
                        MapValueType::Multiple(vec) 
                        => vec.push(value)
                    })
                .or_insert(MapValueType::Single(value));
        } 

        return QueryString{data} 
    }
}