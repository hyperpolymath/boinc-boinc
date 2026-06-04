// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{Expr, Parameter, ResourceKind, ResourceSpec, ResourceType, Type};
use anyhow::{anyhow, Result};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct OblibenyParser;

pub fn parse_file(input: &str) -> Result<Vec<Expr>> {
    let pairs = OblibenyParser::parse(Rule::file, input)
        .map_err(|e| anyhow!("Parse error: {}", e))?;

    let mut exprs = Vec::new();
    for pair in pairs {
        if pair.as_rule() == Rule::form {
            exprs.push(parse_form(pair)?);
        } else if pair.as_rule() == Rule::EOI {
            break;
        }
    }

    Ok(exprs)
}

fn parse_form(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::integer => Ok(Expr::Int(inner.as_str().parse()?)),
        Rule::float => Ok(Expr::Float(inner.as_str().parse()?)),
        Rule::boolean => Ok(Expr::Bool(inner.as_str() == "true")),
        Rule::string => {
            let s = inner.as_str();
            let unquoted = &s[1..s.len() - 1]; // Remove quotes
            Ok(Expr::String(unquoted.to_string()))
        }
        Rule::ident => Ok(Expr::Ident(inner.as_str().to_string())),
        Rule::list => parse_list(inner),
        _ => Err(anyhow!("Unexpected rule: {:?}", inner.as_rule())),
    }
}

fn parse_list(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next();
    if inner.is_none() {
        return Ok(Expr::FunctionCall {
            func: Box::new(Expr::Ident("nil".to_string())),
            args: vec![],
        });
    }

    let inner = inner.unwrap();

    match inner.as_rule() {
        Rule::defun_deploy => parse_defun_deploy(inner),
        Rule::defun_compile => parse_defun_compile(inner),
        Rule::bounded_for => parse_bounded_for(inner),
        Rule::with_capability => parse_with_capability(inner),
        Rule::let_binding => parse_let(inner),
        Rule::if_expr => parse_if(inner),
        Rule::set_var => parse_set(inner),
        Rule::array_get => parse_array_get(inner),
        Rule::array_set => parse_array_set(inner),
        Rule::array_length => parse_array_length(inner),
        Rule::array_literal => parse_array_literal(inner),
        Rule::sleep_ms => parse_sleep_ms(inner),
        Rule::gpio_set => parse_gpio_set(inner),
        Rule::gpio_get => parse_gpio_get(inner),
        Rule::sensor_read => parse_sensor_read(inner),
        Rule::network_send => parse_network_send(inner),
        Rule::program => parse_program(inner),
        Rule::resource_budget => parse_resource_budget(inner),
        Rule::defcap => parse_defcap(inner),
        Rule::function_call => parse_function_call(inner),
        _ => {
            // Default to function call
            parse_function_call(inner)
        }
    }
}

fn parse_defun_deploy(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let params = parse_param_list(inner.next().unwrap())?;

    let mut return_type = None;
    let mut body = Vec::new();

    for pair in inner {
        match pair.as_rule() {
            Rule::type_annotation => {
                return_type = Some(parse_type(pair.into_inner().next().unwrap())?);
            }
            Rule::form => {
                body.push(parse_form(pair)?);
            }
            _ => {}
        }
    }

    Ok(Expr::DefunDeploy {
        name,
        params,
        return_type,
        body,
    })
}

fn parse_defun_compile(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let params = parse_param_list(inner.next().unwrap())?;

    let mut return_type = None;
    let mut body = Vec::new();

    for pair in inner {
        match pair.as_rule() {
            Rule::type_annotation => {
                return_type = Some(parse_type(pair.into_inner().next().unwrap())?);
            }
            Rule::form => {
                body.push(parse_form(pair)?);
            }
            _ => {}
        }
    }

    Ok(Expr::DefunCompile {
        name,
        params,
        return_type,
        body,
    })
}

fn parse_bounded_for(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let var = inner.next().unwrap().as_str().to_string();
    let start = Box::new(parse_form(inner.next().unwrap())?);
    let end = Box::new(parse_form(inner.next().unwrap())?);

    let mut body = Vec::new();
    for pair in inner {
        body.push(parse_form(pair)?);
    }

    Ok(Expr::BoundedFor {
        var,
        start,
        end,
        body,
    })
}

fn parse_with_capability(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let capability = Box::new(parse_form(inner.next().unwrap())?);

    let mut body = Vec::new();
    for pair in inner {
        body.push(parse_form(pair)?);
    }

    Ok(Expr::WithCapability { capability, body })
}

fn parse_let(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let bindings_pair = inner.next().unwrap();
    let bindings = parse_bindings(bindings_pair)?;

    let mut body = Vec::new();
    for pair in inner {
        body.push(parse_form(pair)?);
    }

    Ok(Expr::Let { bindings, body })
}

fn parse_bindings(pair: pest::iterators::Pair<Rule>) -> Result<Vec<(String, Expr)>> {
    let mut bindings = Vec::new();

    for binding in pair.into_inner() {
        let mut inner = binding.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let expr = parse_form(inner.next().unwrap())?;
        bindings.push((name, expr));
    }

    Ok(bindings)
}

fn parse_if(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let condition = Box::new(parse_form(inner.next().unwrap())?);
    let then_branch = Box::new(parse_form(inner.next().unwrap())?);
    let else_branch = Box::new(parse_form(inner.next().unwrap())?);

    Ok(Expr::If {
        condition,
        then_branch,
        else_branch,
    })
}

fn parse_set(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let var = inner.next().unwrap().as_str().to_string();
    let value = Box::new(parse_form(inner.next().unwrap())?);

    Ok(Expr::Set { var, value })
}

fn parse_array_get(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let array = Box::new(parse_form(inner.next().unwrap())?);
    let index = Box::new(parse_form(inner.next().unwrap())?);

    Ok(Expr::ArrayGet { array, index })
}

fn parse_array_set(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let array = Box::new(parse_form(inner.next().unwrap())?);
    let index = Box::new(parse_form(inner.next().unwrap())?);
    let value = Box::new(parse_form(inner.next().unwrap())?);

    Ok(Expr::ArraySet {
        array,
        index,
        value,
    })
}

fn parse_array_length(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next().unwrap();
    Ok(Expr::ArrayLength(Box::new(parse_form(inner)?)))
}

fn parse_array_literal(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let elem_type = parse_type(inner.next().unwrap())?;
    let size = inner.next().unwrap().as_str().parse()?;

    Ok(Expr::ArrayLiteral { elem_type, size })
}

fn parse_sleep_ms(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next().unwrap();
    Ok(Expr::SleepMs(Box::new(parse_form(inner)?)))
}

fn parse_gpio_set(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let device = Box::new(parse_form(inner.next().unwrap())?);
    let value = Box::new(parse_form(inner.next().unwrap())?);

    Ok(Expr::GpioSet { device, value })
}

fn parse_gpio_get(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next().unwrap();
    Ok(Expr::GpioGet(Box::new(parse_form(inner)?)))
}

fn parse_sensor_read(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let inner = pair.into_inner().next().unwrap();
    Ok(Expr::SensorRead(Box::new(parse_form(inner)?)))
}

fn parse_network_send(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let device = Box::new(parse_form(inner.next().unwrap())?);
    let data = Box::new(parse_form(inner.next().unwrap())?);

    Ok(Expr::NetworkSend { device, data })
}

fn parse_program(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let budget = Box::new(parse_resource_budget(inner.next().unwrap())?);

    let mut forms = Vec::new();
    for pair in inner {
        forms.push(parse_form(pair)?);
    }

    Ok(Expr::Program {
        name,
        budget,
        forms,
    })
}

fn parse_resource_budget(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut specs = Vec::new();

    for spec_pair in pair.into_inner() {
        let mut inner = spec_pair.into_inner();
        let kind_str = inner.next().unwrap().as_str();
        let amount: u64 = inner.next().unwrap().as_str().parse()?;

        let kind = match kind_str {
            "time-ms" => ResourceKind::TimeMs,
            "memory-bytes" => ResourceKind::MemoryBytes,
            "network-bytes" => ResourceKind::NetworkBytes,
            "storage-bytes" => ResourceKind::StorageBytes,
            _ => return Err(anyhow!("Unknown resource kind: {}", kind_str)),
        };

        specs.push(ResourceSpec::new(kind, amount));
    }

    Ok(Expr::ResourceBudget { specs })
}

fn parse_defcap(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let params = parse_param_list(inner.next().unwrap())?;
    let description_pair = inner.next().unwrap();
    let desc_str = description_pair.as_str();
    let description = desc_str[1..desc_str.len() - 1].to_string(); // Remove quotes

    Ok(Expr::DefCap {
        name,
        params,
        description,
    })
}

fn parse_function_call(pair: pest::iterators::Pair<Rule>) -> Result<Expr> {
    let mut inner = pair.into_inner();

    let func = Box::new(parse_form(inner.next().unwrap())?);

    let mut args = Vec::new();
    for pair in inner {
        args.push(parse_form(pair)?);
    }

    Ok(Expr::FunctionCall { func, args })
}

fn parse_param_list(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Parameter>> {
    let mut params = Vec::new();

    for param_pair in pair.into_inner() {
        let param = parse_parameter(param_pair)?;
        params.push(param);
    }

    Ok(params)
}

fn parse_parameter(pair: pest::iterators::Pair<Rule>) -> Result<Parameter> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::ident => Ok(Parameter::new(inner.as_str().to_string(), None)),
        _ => {
            // Typed parameter: (name type)
            let mut parts = inner.into_inner();
            let name = parts.next().unwrap().as_str().to_string();
            let ty = parse_type(parts.next().unwrap())?;
            Ok(Parameter::new(name, Some(ty)))
        }
    }
}

fn parse_type(pair: pest::iterators::Pair<Rule>) -> Result<Type> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::simple_type => Ok(match inner.as_str() {
            "int32" => Type::Int32,
            "int64" => Type::Int64,
            "uint32" => Type::Uint32,
            "uint64" => Type::Uint64,
            "float32" => Type::Float32,
            "float64" => Type::Float64,
            "bool" => Type::Bool,
            "string" => Type::String,
            "void" => Type::Void,
            _ => return Err(anyhow!("Unknown type: {}", inner.as_str())),
        }),
        Rule::array_type => {
            let mut parts = inner.into_inner();
            let elem_type = Box::new(parse_type(parts.next().unwrap())?);
            let size = parts.next().unwrap().as_str().parse()?;
            Ok(Type::Array { elem_type, size })
        }
        Rule::capability_type => {
            let resource_pair = inner.into_inner().next().unwrap();
            let resource = match resource_pair.as_str() {
                "uart-tx" => ResourceType::UartTx,
                "uart-rx" => ResourceType::UartRx,
                "gpio" => ResourceType::Gpio,
                "i2c" => ResourceType::I2c,
                "spi" => ResourceType::Spi,
                "sensor-read" => ResourceType::SensorRead,
                "network-send" => ResourceType::NetworkSend,
                "network-recv" => ResourceType::NetworkRecv,
                _ => return Err(anyhow!("Unknown resource type: {}", resource_pair.as_str())),
            };
            Ok(Type::Capability { resource })
        }
        _ => Err(anyhow!("Unknown type rule: {:?}", inner.as_rule())),
    }
}
