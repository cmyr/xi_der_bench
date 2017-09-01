//! I would like to benchmark a few different ways of deserializing RPCs:
//!
//! 1. from a &str, with borrows, right into some type.
//! 2. from a &str, no borrows, into a type
//! 3. from a &str into a Value, into a type

#![feature(test)]

extern crate test;

extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate xi_core_lib;

mod rpc2;
mod rpc3;

use test::Bencher;

use serde_json::Value;

use xi_core_lib::rpc::Request;


//const TEST_JSON: &str = r#"{"method":"client_started","params":{}}
//{"method":"set_theme","params":{"theme_name":"InspiredGitHub"}}
//{"id":0,"method":"new_view","params":{}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"insert","params":{"chars":"\/\/ Copyright 2016 Google Inc. All rights reserved.\n\/\/\n\/\/ Licensed under the Apache License, Version 2.0 (the \"License\");\n\/\/ you may not use this file except in compliance with the License.\n\/\/ You may obtain a copy of the License at\n\/\/\n\/\/     http:\/\/www.apache.org\/licenses\/LICENSE-2.0\n\/\/\n\/\/ Unless required by applicable law or agreed to in writing, software\n\/\/ distributed under the License is distributed on an \"AS IS\" BASIS,\n\/\/ WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n\/\/ See the License for the specific language governing permissions and\n\/\/ limitations under the License."}}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"request_lines","params":[12,13]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"move_to_right_end_of_line","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"move_to_left_end_of_line","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"scroll","params":[3,13]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"move_to_end_of_document","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"move_to_beginning_of_document","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"scroll","params":[0,10]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"move_word_right","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"move_word_left","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"move_word_right_and_modify_selection","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"delete_backward","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"delete_forward","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"insert_newline","params":[]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"scroll","params":[1,11]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"drag","params":[5,34,0]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"click","params":[3,10,0,1]}}
//{"method":"edit","params":{"view_id":"view-id-1","method":"goto_line","params":{"line":1}}}
//{"method":"close_view","params":{"view_id":"view-id-1"}}"#;

const TEST_JSON: &str = r#"{"method":"client_started","params":{}}
{"method":"set_theme","params":{"theme_name":"InspiredGitHub"}}
{"id":0,"method":"new_view","params":{}}
{"method":"edit","params":{"view_id":"view-id-1","method":"insert","params":{"chars":"\/\/ Copyright 2016 Google Inc. All rights reserved.\n\/\/\n\/\/ Licensed under the Apache License, Version 2.0 (the \"License\");\n\/\/ you may not use this file except in compliance with the License.\n\/\/ You may obtain a copy of the License at\n\/\/\n\/\/     http:\/\/www.apache.org\/licenses\/LICENSE-2.0\n\/\/\n\/\/ Unless required by applicable law or agreed to in writing, software\n\/\/ distributed under the License is distributed on an \"AS IS\" BASIS,\n\/\/ WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n\/\/ See the License for the specific language governing permissions and\n\/\/ limitations under the License."}}}
{"method":"edit","params":{"view_id":"view-id-1","method":"request_lines","params":[12,13]}}
{"method":"edit","params":{"view_id":"view-id-1","method":"scroll","params":[3,13]}}
{"method":"edit","params":{"view_id":"view-id-1","method":"move_word_right","params":[]}}
{"method":"edit","params":{"view_id":"view-id-1","method":"move_word_left","params":[]}}
{"method":"edit","params":{"view_id":"view-id-1","method":"delete_backward","params":[]}}
{"method":"edit","params":{"view_id":"view-id-1","method":"delete_forward","params":[]}}
{"method":"edit","params":{"view_id":"view-id-1","method":"insert_newline","params":[]}}
{"method":"edit","params":{"view_id":"view-id-1","method":"drag","params":[5,34,0]}}
{"method":"edit","params":{"view_id":"view-id-1","method":"click","params":[3,10,0,1]}}
{"method":"close_view","params":{"view_id":"view-id-1"}}"#;

#[cfg(test)]
pub fn dict_get_string<'a>(dict: &'a serde_json::Map<String, Value>, key: &str) -> Option<&'a str> {
    dict.get(key).and_then(Value::as_str)
}

#[cfg(test)]
fn parse_rpc_request(json: &Value) -> Option<(Option<&Value>, &str, &Value)> {
    json.as_object().and_then(|req| {
        if let (Some(method), Some(params)) =
            (dict_get_string(req, "method"), req.get("params")) {
                let id = req.get("id");
                Some((id, method, params))
            }
        else { None }
    })
}

#[bench]
fn borrow(b: &mut Bencher) {
    b.iter(|| {
        for json in TEST_JSON.lines() {
            let val = serde_json::from_str::<Value>(json).unwrap();
            match parse_rpc_request(&val) {
                Some((id, method, params)) => {
                    let req = Request::from_json(method, params).unwrap();
                }
                None => eprintln!("invalid RPC request")
            }
        }
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcCall {
    pub method: String,
    pub params: Value,
}

#[bench]
fn own(b: &mut Bencher) {
    b.iter(|| {
        for json in TEST_JSON.lines() {
        let mut val = serde_json::from_str::<Value>(json).unwrap();
        let id = val.as_object_mut().map(|obj| obj.remove("id"));
        let rpc: RpcCall = serde_json::from_value(val).unwrap();
        let req = Request::from_json(&rpc.method, &rpc.params).unwrap();
        }
    })
}

#[bench]
fn serde(b: &mut Bencher) {
	b.iter(|| {
		for json in TEST_JSON.lines() {
			let mut val = serde_json::from_str::<Value>(json).unwrap();
            let id = val.as_object_mut().and_then(|obj| obj.remove("id"));
            let req = if id.is_some() {
                serde_json::from_value::<rpc2::CoreRequest>(val).err()
            } else {
                serde_json::from_value::<rpc2::CoreNotification>(val).err()
            };
            match *&req {
                Some(ref e) => eprintln!("{:?}\n{}", e, json),
                None => (),
            }
            assert!(req.is_none());
        }
    })
}

#[bench]
fn future_serde(b: &mut Bencher) {
	b.iter(|| {
		for json in TEST_JSON.lines() {
			//let mut val = serde_json::from_str::<Value>(json).unwrap();
            //let id = val.as_object_mut().and_then(|obj| obj.remove("id"));
            //let req = if id.is_some() {
                //serde_json::from_value::<rpc2::CoreRequest>(val).err()
            //} else {
                //serde_json::from_value::<rpc2::CoreNotification>(val).err()
            //};
            let req = serde_json::from_str::<rpc3::CoreNotification>(&json).err();
            match *&req {
                Some(ref e) => eprintln!("{:?}\n{}", e, json),
                None => (),
            }
            assert!(req.is_none());
        }
    })
}

#[cfg(test)]
mod test_tagging {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "snake_case")]
    enum ExternalTag {
        Blue,
        Other(String),
        Yellowish { how_yellow: f64 },
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "snake_case")]
    #[serde(tag = "method")]
    enum InternalTag {
        Blue,
        Other(String),
        Yellowish { how_yellow: f64 },
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "snake_case")]
    #[serde(tag = "method", content = "params")]
    enum AdjacentlyTag {
        Blue,
        Other(String),
        Yellowish { how_yellow: f64 },
    }

    #[test]
    fn test_external() {
        let blue = r#"{"blue": {}}"#;
        let yellow = r#"{"yellowish": {"how_yellow": 1.4}}"#;
        let x = serde_json::from_str::<ExternalTag>(&blue).unwrap();
        let x = serde_json::from_str::<ExternalTag>(&yellow).unwrap();
    }

    #[test]
    fn test_internal() {
        let blue = r#"{"method": "blue"}"#;
        let yellow = r#"{"method": "yellowish", "how_yellow": 1.4}"#;
        let x = serde_json::from_str::<InternalTag>(&blue);
        assert!(x.is_ok());
        let x = serde_json::from_str::<InternalTag>(&yellow);
        assert!(x.is_ok());
    }

    #[test]
    fn test_adjacent() {
        let blue = r#"{"method": "blue", "params": {}}"#;
        let yellow = r#"{"method": "yellowish", "params": {"how_yellow": 1.4}}"#;
        let x = serde_json::from_str::<AdjacentlyTag>(&blue).unwrap();
        let x = serde_json::from_str::<AdjacentlyTag>(&yellow).unwrap();
    }
}
