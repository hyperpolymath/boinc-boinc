// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Type {
    Int32,
    Int64,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Bool,
    String,
    Void,
    Array {
        elem_type: Box<Type>,
        size: usize,
    },
    Capability {
        resource: ResourceType,
    },
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    UartTx,
    UartRx,
    Gpio,
    I2c,
    Spi,
    SensorRead,
    NetworkSend,
    NetworkRecv,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int32 => write!(f, "int32"),
            Type::Int64 => write!(f, "int64"),
            Type::Uint32 => write!(f, "uint32"),
            Type::Uint64 => write!(f, "uint64"),
            Type::Float32 => write!(f, "float32"),
            Type::Float64 => write!(f, "float64"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Void => write!(f, "void"),
            Type::Array { elem_type, size } => write!(f, "(array {} {})", elem_type, size),
            Type::Capability { resource } => write!(f, "(capability {:?})", resource),
            Type::Function { params, return_type } => {
                write!(f, "(-> ")?;
                for param in params {
                    write!(f, "{} ", param)?;
                }
                write!(f, "{})", return_type)
            }
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResourceType::UartTx => write!(f, "uart-tx"),
            ResourceType::UartRx => write!(f, "uart-rx"),
            ResourceType::Gpio => write!(f, "gpio"),
            ResourceType::I2c => write!(f, "i2c"),
            ResourceType::Spi => write!(f, "spi"),
            ResourceType::SensorRead => write!(f, "sensor-read"),
            ResourceType::NetworkSend => write!(f, "network-send"),
            ResourceType::NetworkRecv => write!(f, "network-recv"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_annotation: Option<Type>,
}

impl Parameter {
    pub fn new(name: String, type_annotation: Option<Type>) -> Self {
        Self {
            name,
            type_annotation,
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.type_annotation {
            Some(ty) => write!(f, "({} {})", self.name, ty),
            None => write!(f, "{}", self.name),
        }
    }
}
