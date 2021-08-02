#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use mut_static::MutStatic;


#[derive(Debug)]
pub struct Foo {
    x: i32,
}

pub static FOOS1: [Foo; 2] = [
    Foo {
        x: 1,
    },
    Foo {
        x: 1000,
    },
];

pub static FOOS2: [Foo; 2] = [
    Foo {
        x: 2,
    },
    Foo {
        x: 200,
    },
];

pub static FOOS3: [Foo; 2] = [
    Foo {
        x: 3,
    },
    Foo {
        x: 30,
    },
];


type Foos<'a> = Option<&'a [Foo]>;

#[derive(Debug)]
pub struct MyStruct<'a> { 
    value: Foos<'a>
}
impl <'a>MyStruct<'a> {
    pub fn new(v: Option<&'a [Foo]>) -> Self{
        MyStruct { value: v }
    }
    pub fn getvalue(&self) -> Foos { self.value }
    pub fn setvalue(&mut self, v: Option<&'a [Foo]>) { self.value = v }
}

lazy_static! {
    static ref MY_GLOBAL_STATE: MutStatic<MyStruct<'static>> = MutStatic::new();
}

fn main() {
    // Here, I call .set on the MutStatic to put data inside it.
    // This can fail.
    MY_GLOBAL_STATE.set(MyStruct::new(Some(&FOOS1))).unwrap();
    {
        // Using the global state immutably is easy...
        println!("Before mut: {:?}", 
                 MY_GLOBAL_STATE.read().unwrap().getvalue());
    }
    {
         // Using it mutably is too...
         let mut mut_handle = MY_GLOBAL_STATE.write().unwrap();
         mut_handle.setvalue(Some(&FOOS2));
         println!("Changed value to Some(&FOOS2).");
    } 
    {
        // As long as there's a scope change we can get the 
        // immutable version again...
        println!("After mut: {:?}", 
                 MY_GLOBAL_STATE.read().unwrap().getvalue());
    }
    {
        // But beware! Anything can change global state!
        foo();
        println!("After foo: {:?}", 
                 MY_GLOBAL_STATE.read().unwrap().getvalue());
    }
 
}

// Note that foo takes no parameters
fn foo() {
    let mut mut_handle = 
        MY_GLOBAL_STATE.write().unwrap();
    mut_handle.setvalue(Some(&FOOS3));
}
