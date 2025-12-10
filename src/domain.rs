use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedDnsEntry {
    pub name: String,
    pub primary: String,
    pub     secondary: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DnsProvider {
    Electro {
        primary: String,
        secondary: String,
    },
    Radar {
        primary: String,
        secondary: String,
    },
    Shekan {
        primary: String,
        secondary: String,
    },
    Bogzar {
        primary: String,
        secondary: String,
    },
    Quad9 {
        primary: String,
        secondary: String,
    },
    Custom {
        primary: String,
        secondary: String,
    },
    Saved {
        name: String,
        primary: String,
        secondary: String,
    },
}

impl DnsProvider {
    pub fn electro() -> Self {
        Self::Electro {
            primary: "78.157.42.100".to_string(),
            secondary: "78.157.42.101".to_string(),
        }
    }

    pub fn radar() -> Self {
        Self::Radar {
            primary: "10.202.10.10".to_string(),
            secondary: "10.202.10.11".to_string(),
        }
    }

    pub fn shekan() -> Self {
        Self::Shekan {
            primary: "178.22.122.100".to_string(),
            secondary: "185.51.200.2".to_string(),
        }
    }

    pub fn bogzar() -> Self {
        Self::Bogzar {
            primary: "185.55.226.26".to_string(),
            secondary: "185.55.225.25".to_string(),
        }
    }

    pub fn quad9() -> Self {
        Self::Quad9 {
            primary: "9.9.9.9".to_string(),
            secondary: "149.112.112.112".to_string(),
        }
    }

    pub fn custom(primary: String, secondary: String) -> Self {
        Self::Custom { primary, secondary }
    }

    pub fn saved(name: String, primary: String, secondary: String) -> Self {
        Self::Saved {
            name,
            primary,
            secondary,
        }
    }

    pub fn get_servers(&self) -> (String, String) {
        match self {
            DnsProvider::Electro { primary, secondary }
            | DnsProvider::Radar { primary, secondary }
            | DnsProvider::Shekan { primary, secondary }
            | DnsProvider::Bogzar { primary, secondary }
            | DnsProvider::Quad9 { primary, secondary }
            | DnsProvider::Custom { primary, secondary }
            | DnsProvider::Saved {
                primary, secondary, ..
            } => (primary.clone(), secondary.clone()),
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            DnsProvider::Electro { .. } => "Electro".to_string(),
            DnsProvider::Radar { .. } => "Radar".to_string(),
            DnsProvider::Shekan { .. } => "Shekan".to_string(),
            DnsProvider::Bogzar { .. } => "Bogzar".to_string(),
            DnsProvider::Quad9 { .. } => "Quad9".to_string(),
            DnsProvider::Custom { .. } => "Custom".to_string(),
            DnsProvider::Saved { name, .. } => name.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DnsOperation {
    Set(DnsProvider),
    Clear,
    Test,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationResult {
    Success(String),
    Error(String),
    Warning(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AppState {
    #[default]
    Idle,
    Processing,
    Success(String),
    Error(String),
    Warning(String),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum DnsState {
    Static(Vec<String>),
    Dhcp,
    #[default]
    None,
}

impl Default for DnsProvider {
    fn default() -> Self {
        DnsProvider::electro()
    }
}
