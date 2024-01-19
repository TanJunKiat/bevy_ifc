use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default, Resource)]
pub struct IfcResource{
    pub item: HashMap<i32, HashMap<String, String>>,
}

pub fn parse_ifc(ifc_string: String
){

}

