mod utils;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use zvariant::{
    serialized::{Context, Data},
    to_bytes, Type, LE,
};

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() -> String {
//     return "PgLTree, pg-ltree!".to_string();
// }

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Type)]
pub struct MpTree {
    nodes: Vec<MpNode>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Type)]
pub struct MpNode {
    // node relationship
    parent: Option<NodeIndex>,
    previous_sibling: Option<NodeIndex>,
    next_sibling: Option<NodeIndex>,
    first_child: Option<NodeIndex>,
    last_child: Option<NodeIndex>,

    //data
    label: String,
    path: String,
    id: i32,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Type)]
pub struct NodeIndex {
    index: u32,
}

#[wasm_bindgen]
impl MpTree {
    // pub fn new(first_name: String, last_name: String) -> MpTree {
    //     return MpTree {

    //     };
    // }

    // pub fn set_data(&mut self, label: String, id: i32) {
    //     self.label = name;
    // }

    // pub fn get_data(&self) {
    //     self.label = name;
    // }

    pub fn new_node(&mut self) -> NodeIndex {
        let free_index: u32 = self.nodes.len() as u32;

        //TODO: constructor
        self.nodes.push();

        NodeIndex { index: free_index }
    }

    //TODO: catch error
    pub fn ser(&self) -> Vec<u8> {
        let ctxt = Context::new_gvariant(LE, 0);

        (*to_bytes(ctxt, &self).unwrap()).to_vec()
    }

    //TODO: catch error
    pub fn de(&mut self, bytes: Vec<u8>) {
        let ctxt = Context::new_gvariant(LE, 0);
        let encode = Data::new(bytes, ctxt);

        *self = encode.deserialize::<MpTree>().unwrap().0;
    }
}

#[wasm_bindgen]
impl MpNode {
    pub fn new(label: String, path: String, id: i32) -> MpNode {}
    pub fn greet(&self) -> String {
        return format!("account {}{}", self.label, self.id);
    }
    pub fn set_data(&mut self, label: String, id: i32) {
        self.label = label;
        self.id = id;
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }
}

#[wasm_bindgen]
pub fn decode(bytes: Vec<u8>) -> MpTree {
    let ctxt = Context::new_gvariant(LE, 0);
    let encode = Data::new(bytes, ctxt);

    return encode.deserialize::<MpTree>().unwrap().0;
}

#[cfg(test)]
mod tests {
    // use postcard::to_vec;
    use zvariant::{serialized::Context, to_bytes, LE};

    use super::*;
    #[test]
    fn struct_ser_de() {
        let ctx = Context::new_gvariant(LE, 0);
        let obj = MpTree::new("Steve".to_string(), "Sujo".to_string());
        let encode = to_bytes(ctx, &obj).unwrap();
        let decode: MpTree = encode.deserialize().unwrap().0;

        assert_eq!(obj.first_name, decode.first_name);
    }

    #[test]
    fn impl_ser_de() {
        let obj = MpTree::new("Steve".to_string(), "Sujo".to_string());

        let bytes = obj.ser();
        let decode = decode(bytes);

        assert_eq!(obj.first_name, decode.first_name);
    }
}
