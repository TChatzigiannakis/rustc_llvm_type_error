fn main() {
    let callbacks = Callbacks::new();
    callbacks.do_nothing(|x| &x.field);
}

pub struct Callbacks {
    field: Vec<Box<Fn(&u8)>>,
}

impl Callbacks {
    pub fn new() -> Callbacks {
        Callbacks { field: Vec::new() }
    }

    pub fn do_nothing<T>(&self, _selector: fn(&Callbacks) -> &Vec<Box<Fn(T)>>) {}
}
