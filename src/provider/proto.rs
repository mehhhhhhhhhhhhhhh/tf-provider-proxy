use std::collections::HashMap;
use std::mem::transmute;

pub(crate) mod generated {
    tonic::include_proto!("tfplugin5");
}

use serde::ser::SerializeMap;
use generated::Schema;
use generated::schema::{Attribute,Block,NestedBlock};
use generated::schema::nested_block::NestingMode;

pub use generated::get_provider_schema;
pub use generated::provider_client::ProviderClient;

impl serde::ser::Serialize for Schema {
    fn serialize<S>(&self, ser: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where S: serde::Serializer {
        let mut map = ser.serialize_map(Some(2))?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("main_block", &self.block)?;
        map.end()
    }
}
impl serde::ser::Serialize for Block {
    fn serialize<S>(&self, ser: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where S: serde::Serializer {
        let mut map = ser.serialize_map(Some(3))?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("attrs", &self.attributes)?;
        map.serialize_entry("blocks", &self.block_types)?;
        map.end()
    }
}
impl serde::ser::Serialize for NestedBlock {
    fn serialize<S>(&self, ser: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where S: serde::Serializer {
        let mut map = ser.serialize_map(Some(3))?;
        map.serialize_entry("name", &self.type_name)?;
        map.serialize_entry("block", &self.block)?;
        map.serialize_entry("nesting_mode", &(unsafe { transmute::<i32, NestingMode>(self.nesting) }).as_str_name().to_lowercase())?;
        map.serialize_entry("min_items", &self.min_items)?;
        map.serialize_entry("max_items", &self.max_items)?;
        map.end()
    }
}
impl serde::ser::Serialize for Attribute {
    fn serialize<S>(&self, ser: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where S: serde::Serializer {
        let mut map = ser.serialize_map(Some(3))?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("type", &serde_json::from_str::<serde_json::Value>(std::str::from_utf8(&self.r#type).unwrap()).unwrap())?;
        if !self.description.is_empty() {
            map.serialize_entry("description", &self.description)?;
        }
        map.serialize_entry("required", &self.required)?;
        map.serialize_entry("optional", &self.optional)?;
        map.serialize_entry("computed", &self.computed)?;
        if self.sensitive {
            map.serialize_entry("sensitive", &self.sensitive)?;
        }
        map.end()
    }
}
impl serde::ser::Serialize for get_provider_schema::Response {
    fn serialize<S>(&self, ser: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where S: serde::Serializer {
        // TODO ordering? or otherwise avoid constructing a map
        HashMap::from([
            //("provider", self.provider),
            ("resources", &self.resource_schemas),
            ("datas", &self.data_source_schemas)
        ]).serialize(ser)
    }
}
