use std::error::Error;
use std::fmt::Debug;

use crate::errors::GeneralError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArgDataType {
    String,
    Bool,

    I32,
    I64,

    U32,
    U64,

    F32,
    F64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArgType {
    BoolFlag { default: bool },
    Required,
    Optional { default: String },
}

pub struct Arg {
    pub name: String,
    pub alias: Option<String>,
    pub help: String,
    pub data_type: ArgDataType,
    pub ty: ArgType,
}

impl Arg {
    fn new(
        name: &str,
        alias: Option<&str>,
        help: &str,
        data_type: ArgDataType,
        ty: ArgType,
    ) -> Self {
        Arg {
            name: name.to_owned(),
            alias: alias.map(|s| s.to_owned()),
            help: help.to_owned(),
            ty,
            data_type,
        }
    }

    pub fn dashed_name(&self) -> String {
        format!("--{}", self.name)
    }

    pub fn dashed_alias(&self) -> Option<String> {
        self.alias.as_ref().map(|s| format!("-{}", s))
    }

    pub fn matches(&self, string: &str) -> bool {
        self.dashed_name().eq_ignore_ascii_case(string)
            || self
                .dashed_alias()
                .as_ref()
                .map(|s| s.eq_ignore_ascii_case(string))
                .unwrap_or(false)
    }

    pub fn try_get_match_index(&self, args: &[String]) -> Option<usize> {
        args.iter().position(|value| {
            self.dashed_name().eq_ignore_ascii_case(value)
                || self
                    .dashed_alias()
                    .map(|alias| alias.eq_ignore_ascii_case(value))
                    .unwrap_or(false)
        })
    }
}

pub struct ArgCollection {
    args: Vec<Arg>,
    strings: Vec<String>,
}

impl<'a> ArgCollection {
    pub fn new() -> Self {
        ArgCollection {
            args: Vec::new(),
            strings: vec![],
        }
    }

    pub fn add_arg(
        &mut self,
        name: &str,
        alias: Option<&str>,
        help: &str,
        data_type: ArgDataType,
        ty: ArgType,
    ) -> &Arg {
        self.args.push(Arg::new(name, alias, help, data_type, ty));
        self.args.last().unwrap()
    }

    pub fn add_optional(
        &mut self,
        name: &str,
        alias: Option<&str>,
        help: &str,
        data_type: ArgDataType,
        default: &str,
    ) -> &Arg {
        self.add_arg(
            name,
            alias,
            help,
            data_type,
            ArgType::Optional {
                default: default.to_owned(),
            },
        )
    }

    pub fn add_required(
        &mut self,
        name: &str,
        alias: Option<&str>,
        help: &str,
        data_type: ArgDataType,
    ) -> &Arg {
        self.add_arg(name, alias, help, data_type, ArgType::Required)
    }

    pub fn add_bool_flag(
        &mut self,
        name: &str,
        alias: Option<&str>,
        help: &str,
        default: bool,
    ) -> &Arg {
        self.add_arg(
            name,
            alias,
            help,
            ArgDataType::Bool,
            ArgType::BoolFlag { default },
        )
    }

    pub fn bind(&mut self, strings: Vec<String>) {
        self.strings = strings;
    }

    pub fn get_arg_ref(&self, name: &str) -> Option<&Arg> {
        self.args.iter().find(|a| a.name == name)
    }

    pub fn bind_env_args(&mut self) {
        // dbg!(std::env::args().collect::<Vec<String>>());
        self.strings = std::env::args().collect();
    }

    pub fn print_help_and_exit(&self, error: &str) -> ! {
        let mut output = String::new();
        output.push_str(&format!("Invalid Usage: {}", error));

        for arg in self.args.iter() {
            output.push('\n');
            if let Some(alias) = &arg.alias {
                output.push_str(&format!("-{} | ", alias));
            }
            output.push_str(&format!("--{} => {}", arg.name, arg.help));
            output.push_str(&match arg.ty {
                ArgType::Optional { ref default } => format!(r#"  || DEFAULT: {}"#, default),
                ArgType::Required => format!(r#"  || REQUIRED"#),
                _ => String::default(),
            });
        }

        println!("{}", output);
        std::process::exit(1);
    }

    pub fn get_arg_by_name(&self, name: &str) -> Option<&Arg> {
        self.args.iter().find(
            |a| a.name.eq_ignore_ascii_case(name), /*|| match &a.alias {
                                                       Some(alias) => alias.eq_ignore_ascii_case(name),
                                                       None => false
                                                   }*/
        )
    }

    pub fn get_arg_by_name_or_error(&self, name: &str) -> Result<&Arg, Box<dyn Error>> {
        self.get_arg_by_name(name).ok_or(GeneralError::boxed(
            "ArgNotFoundError",
            &format!("Could not find any arg by the name: {}", name),
        ))
    }

    pub fn parse_bool_flag(&self, name: &str) -> Result<bool, Box<dyn Error>> {
        let arg = self.get_arg_by_name_or_error(name)?;
        Ok(self.strings.iter().any(|x| arg.matches(x))
            || match arg.ty {
                ArgType::BoolFlag { default } => default,
                _ => false,
            })
    }

    pub fn parse_string(&self, name: &str) -> Result<String, Box<dyn Error>> {
        let arg = self.get_arg_by_name_or_error(name)?;

        let default = match &arg.ty {
            ArgType::Optional { default } => Some(default.to_owned()),
            ArgType::BoolFlag { default } => Some(default.to_string()),
            _ => None,
        };

        let match_index = arg.try_get_match_index(&self.strings);
        if match_index.is_none() {
            if default.is_some() {
                return Ok(default.unwrap());
            }
            return Err(GeneralError::boxed(
                "ArgMatchNotFound",
                &format!("Could not find any match for arg: {} ", arg.name),
            ));
        }

        let after_match = self
            .strings
            .get((match_index.unwrap())..)
            .map(|x| x.to_vec())
            .unwrap_or(vec![arg.dashed_name()]);
        match after_match.len() {
            1 => match default {
                Some(value) => Ok(value),
                None => Err(GeneralError::boxed(
                    "MissingValueForArgument",
                    &format!("missing required value for arg: {} ", arg.name),
                )),
            },
            2.. => Ok(after_match
                .iter()
                .skip(1)
                .take_while(|x| !x.starts_with('-'))
                .map(|x| x.to_owned())
                .collect::<Vec<_>>()
                .join(" ")),
            _ => Err(GeneralError::boxed(
                "Error",
                &format!("this should not happen"),
            )),
        }
    }

    pub fn parse_i32(&self, name: &str) -> Result<i32, Box<dyn Error>> {
        let string_match = self.parse_string(name)?;
        match string_match.parse::<i32>() {
            Ok(x) => Ok(x),
            Err(_) => Err(GeneralError::boxed(
                "InvalidIntError",
                &format!(r#"Invalid value for int32: "{}" "#, string_match),
            )),
        }
    }

    pub fn parse_i64(&self, name: &str) -> Result<i64, Box<dyn Error>> {
        let string_match = self.parse_string(name)?;
        match string_match.parse::<i64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(GeneralError::boxed(
                "InvalidIntError",
                &format!(r#"Invalid value for int64: "{}""#, string_match),
            )),
        }
    }

    pub fn parse_u32(&self, name: &str) -> Result<u32, Box<dyn Error>> {
        let string_match = self.parse_string(name)?;
        match string_match.parse::<u32>() {
            Ok(x) => Ok(x),
            Err(_) => Err(GeneralError::boxed(
                "InvalidIntError",
                &format!(r#"Invalid value for unsigned int32: "{}""#, string_match),
            )),
        }
    }

    pub fn parse_u64(&self, name: &str) -> Result<u64, Box<dyn Error>> {
        let string_match = self.parse_string(name)?;
        match string_match.parse::<u64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(GeneralError::boxed(
                "InvalidIntError",
                &format!(r#"Invalid value for unsigned int64: "{}""#, string_match),
            )),
        }
    }

    pub fn parse_f32(&self, name: &str) -> Result<f32, Box<dyn Error>> {
        let string_match = self.parse_string(name)?;
        match string_match.parse::<f32>() {
            Ok(x) => Ok(x),
            Err(_) => Err(GeneralError::boxed(
                "InvalidIntError",
                &format!(r#"Invalid value for float32: "{}""#, string_match),
            )),
        }
    }

    pub fn parse_f64(&self, name: &str) -> Result<f64, Box<dyn Error>> {
        let string_match = self.parse_string(name)?;
        match string_match.parse::<f64>() {
            Ok(x) => Ok(x),
            Err(_) => Err(GeneralError::boxed(
                "InvalidIntError",
                &format!(r#"Invalid value for float64: "{}""#, string_match),
            )),
        }
    }

    pub fn check(&self) {
        for arg in self.args.iter() {
            if arg.ty != ArgType::Required {
                continue;
            }

            let matched = match self.parse_string(&arg.name) {
                Ok(res) => res,
                Err(_) => {
                    self.print_help_and_exit(&format!(
                        "Missing require argument: {}",
                        arg.dashed_name()
                    ));
                }
            };

            match arg.data_type {
                ArgDataType::Bool => {
                    if matched.parse::<bool>().is_err() {
                        self.print_help_and_exit(&format!(
                            "Invalid bool (must be `true` or `false`): {}",
                            matched
                        ));
                    }
                }

                ArgDataType::I32 => {
                    if matched.parse::<i32>().is_err() {
                        self.print_help_and_exit(&format!("Invalid int32 number: {}", matched));
                    }
                }

                ArgDataType::I64 => {
                    if matched.parse::<i32>().is_err() {
                        self.print_help_and_exit(&format!("Invalid int64 number: {}", matched));
                    }
                }

                ArgDataType::U32 => {
                    if matched.parse::<u32>().is_err() {
                        self.print_help_and_exit(&format!(
                            "Invalid number (must be 0 or greater): {}",
                            matched
                        ));
                    }
                }

                ArgDataType::U64 => {
                    if matched.parse::<u64>().is_err() {
                        self.print_help_and_exit(&format!(
                            "Invalid number (must be 0 or greater): {}",
                            matched
                        ));
                    }
                }

                ArgDataType::F32 => {
                    if matched.parse::<f32>().is_err() {
                        self.print_help_and_exit(&format!("Invalid decimal number: {}", matched));
                    }
                }

                ArgDataType::F64 => {
                    if matched.parse::<f32>().is_err() {
                        self.print_help_and_exit(&format!("Invalid decimal number: {}", matched));
                    }
                }

                _ => {}
            }
        }
    }
}
