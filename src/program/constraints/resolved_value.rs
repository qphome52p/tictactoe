//! The in memory stored value for a defined name in a resolved aleo program.
//!
//! @file resolved_value.rs
//! @author Collin Chin <collin@aleo.org>
//! @date 2020

use crate::program::types::{Function, Struct, StructMember, Type, Variable};

use snarkos_models::curves::{Field, PrimeField};
use snarkos_models::gadgets::{utilities::boolean::Boolean, utilities::uint32::UInt32};
use std::fmt;

#[derive(Clone)]
pub enum ResolvedValue<F: Field + PrimeField> {
    U32(UInt32),
    FieldElement(F),
    Boolean(Boolean),
    Array(Vec<ResolvedValue<F>>),
    StructDefinition(Struct<F>),
    StructExpression(Variable<F>, Vec<StructMember<F>>),
    Function(Function<F>),
    Return(Vec<ResolvedValue<F>>), // add Null for function returns
}

impl<F: Field + PrimeField> ResolvedValue<F> {
    pub(crate) fn match_type(&self, ty: &Type<F>) -> bool {
        match (self, ty) {
            (ResolvedValue::U32(ref _a), Type::U32) => true,
            (ResolvedValue::FieldElement(ref _a), Type::FieldElement) => true,
            (ResolvedValue::Boolean(ref _a), Type::Boolean) => true,
            (ResolvedValue::Array(ref arr), Type::Array(ref _ty, ref len)) => arr.len() == *len, // todo: add array types
            (
                ResolvedValue::StructExpression(ref actual_name, ref _members),
                Type::Struct(ref expected_name),
            ) => actual_name == expected_name,
            (ResolvedValue::Return(ref values), ty) => {
                let mut res = true;
                for value in values {
                    res &= value.match_type(ty)
                }
                res
            }
            (_, _) => false,
        }
    }
}

impl<F: Field + PrimeField> fmt::Display for ResolvedValue<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResolvedValue::U32(ref value) => write!(f, "{}", value.value.unwrap()),
            ResolvedValue::FieldElement(ref value) => write!(f, "{}", value),
            ResolvedValue::Boolean(ref value) => write!(f, "{}", value.get_value().unwrap()),
            ResolvedValue::Array(ref array) => {
                write!(f, "[")?;
                for (i, e) in array.iter().enumerate() {
                    write!(f, "{}", e)?;
                    if i < array.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ResolvedValue::StructExpression(ref variable, ref members) => {
                write!(f, "{} {{", variable)?;
                for (i, member) in members.iter().enumerate() {
                    write!(f, "{}: {}", member.variable, member.expression)?;
                    if i < members.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            ResolvedValue::Return(ref values) => {
                write!(f, "Return values : [")?;
                for (i, value) in values.iter().enumerate() {
                    write!(f, "{}", value)?;
                    if i < values.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            ResolvedValue::StructDefinition(ref _definition) => {
                unimplemented!("cannot return struct definition in program")
            }
            ResolvedValue::Function(ref _function) => {
                unimplemented!("cannot return function definition in program")
            } // _ => unimplemented!("display not impl for value"),
        }
    }
}
