use crate::Deserialize;

pub struct Subdomain {
    domain: String,
    open_ports: Vec<Port>
}

pub struct Port {
    port: u16,
    is_open: bool
}

#[derive(Debug, Deserialize, Clone)]
pub struct CrtShEntry {
    pub name_value: String,
}

