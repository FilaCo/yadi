use crate::prelude::{Container, Injectable, YadiResult};

pub trait Builder {
    fn register_singleton<T: Injectable>(&mut self) -> &mut Self;
    fn build(self) -> YadiResult<impl Container>;
}
