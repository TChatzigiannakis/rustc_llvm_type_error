fn main() {
    let callbacks = Callbacks::new();
    callbacks.type_error(|x| &x.field);
}

pub struct Callbacks {
    field: Vec<Box<Fn(&u8)>>,
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks { field: Vec::new() }
    }

    pub fn no_type_error(&self, _selector: fn(&Callbacks) -> &Vec<Box<Fn(&u8)>>) {}
    pub fn type_error<T>(&self, _selector: fn(&Callbacks) -> &Vec<Box<Fn(T)>>) {}
}
