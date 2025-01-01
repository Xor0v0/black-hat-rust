use serde::Deserialize;

#[derive(Debug)]
pub struct Port {
    pub port: u16,
    pub is_open: bool,
}

#[derive(Debug)]
pub struct SubDomain {
    pub domain: String,
    pub open_ports: Vec<Port>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    pub name_value: String,
}
