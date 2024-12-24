//! # rust_example
//!
//! `rust_example` 项目是基于文档 [通过例子学 Rust 中文版](https://rustwiki.org/zh-CN/rust-by-example/index.html) 的学习示例集合
//!
//! 根据文档章节为 `lib crate` 创建对应的 `module`
//!
//! 并在 `bin` 目录下创建对应的 `bin crate` 中执行对应的 `module`

#![crate_name = "rust_example"]

pub mod c1_hello_world;
pub mod c2_primitives;
pub mod c3_custom_types;
pub mod c4_variable_bindings;
pub mod c5_types;
pub mod c6_conversion;
pub mod c7_expressions;
pub mod c8_flow_of_control;
pub mod c9_functions;

pub mod c10_modules;
pub mod c11_crates;
pub mod c12_cargo;
pub mod c13_attributes;
pub mod c14_generics;
pub mod c15_scoping_rules;
pub mod c16_traits;
pub mod c17_macro_rules;
pub mod c18_error_handling;
pub mod c19_std_library_types;

pub mod c20_std_misc;
pub mod c21_testing;
pub mod c22_unsafe_operations;
pub mod c23_compatibility;
pub mod c24_meta;
