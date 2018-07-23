#![feature(nll, rust_2018_preview)]

use lifetime_gats::{LifetimeCast, Reference, ReferenceMut};
use std::mem;

trait Custom {
    fn custom_exists(&self) {
        println!("custom exists!");
    }
}

impl Custom for &'static u32 {}
impl Custom for &'static mut u32 {}
impl Custom for SpecialRef<'static> {}
impl Custom for SpecialRefMut<'static> {}

trait Foo {
    type Data: Custom;
    type DataMut: Custom;

    fn method(&'this self) -> Reference<'this, Self::Data>;
    fn method_mut(&'this mut self) -> ReferenceMut<'this, Self::DataMut>;
}

#[allow(unused)]
struct Normal(u32);

impl Foo for Normal {
    type Data = &'static u32;
    type DataMut = &'static mut u32;

    fn method(&'this self) -> Reference<'this, &'static u32> {
        Reference::new(&self.0)
    }

    fn method_mut(&'this mut self) -> ReferenceMut<'this, &'static mut u32> {
        ReferenceMut::new(&mut self.0)
    }
}

#[allow(unused)]
struct Special(u32);

impl Foo for Special {
    type Data = SpecialRef<'static>;
    type DataMut = SpecialRefMut<'static>;

    fn method(&'this self) -> Reference<'this, Self::Data> {
        Reference::new(SpecialRef(&self.0))
    }

    fn method_mut(&'this mut self) -> ReferenceMut<'this, Self::DataMut> {
        ReferenceMut::new(SpecialRefMut(&mut self.0))
    }
}

struct SpecialRef<'a>(&'a u32);
struct SpecialRefMut<'a>(&'a mut u32);

unsafe impl LifetimeCast<'b> for SpecialRef<'a> {
    type Target = SpecialRef<'b>;
    unsafe fn cast(self) -> SpecialRef<'b> {
        mem::transmute(self)
    }
    unsafe fn cast_reference(&self) -> &SpecialRef<'b> {
        mem::transmute(self)
    }
    unsafe fn cast_reference_mut(&mut self) -> &mut Self::Target {
        mem::transmute(self)
    }
}

unsafe impl LifetimeCast<'b> for SpecialRefMut<'a> {
    type Target = SpecialRefMut<'b>;
    unsafe fn cast(self) -> Self::Target {
        mem::transmute(self)
    }
    unsafe fn cast_reference(&self) -> &Self::Target {
        mem::transmute(self)
    }
    unsafe fn cast_reference_mut(&mut self) -> &mut Self::Target {
        mem::transmute(self)
    }
}

struct Bar<'a>(&'a Vec<u32>);

impl Bar<'a> {
    fn r(&'x self) -> Reference<'x, &'static Vec<u32>> {
        Reference::new(self.0)
    }

    fn foo(&'x self) -> &'x Vec<u32> {
        &*self.r()
    }
}

fn main() {
    let x = vec![10];
    Bar(&x).foo();
}
