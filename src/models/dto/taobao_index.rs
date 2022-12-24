use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaobaoInDTO {
    index: Option<i32>,
}


#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaobaoInValueDTO {
    index: Option<i32>,
    value:Option<i32>,
}
impl TaobaoInValueDTO {
    pub fn new(index: i32, value:i32)->Self {
        TaobaoInValueDTO{
            index: Some(index),
            value: Some(value),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaobaoListInDTO {
    indexs: Option<Vec<i32>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaobaoListInValueDTO {
    indexs: Option<Vec<i32>>,
    values:Option<Vec<i32>>,
}
impl TaobaoListInValueDTO {
    pub fn new()->Self {
        TaobaoListInValueDTO{
            indexs: Some(vec![]),
            values: Some(vec![]),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct TaobaoPayInValueDTO {
    index: Option<i32>,
    values:Option<f32>,
}

impl TaobaoPayInValueDTO {
    pub fn new(index: i32, values:f32)->Self {
        TaobaoPayInValueDTO{
            index: Some(index),
            values: Some(values),
        }
    }
}
