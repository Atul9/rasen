#![feature(test)]

#[macro_use]
extern crate rasen;
extern crate test;

mod data;

use test::Bencher;
use data::*;

#[bench]
fn bench_construct_basic_frag(b: &mut Bencher) {
    b.iter(|| construct_basic_frag());
}

#[bench]
fn bench_construct_basic_vert(b: &mut Bencher) {
    b.iter(|| construct_basic_vert());
}
