macro_rules! newtype_id (
    ($name: ident : $type: ty) => {
        #[derive(Copy,Hash,PartialEq,Eq,Clone,Debug,Default,Serialize,Deserialize,DieselNewType)]
        pub struct $name(pub $type);

        impl Inc for $name {
            fn inc(&mut self) {
                self.0 += 1;
            }
        }
    };
);

macro_rules! newtype (
    ($name: ident : $type: ty) => {
        #[derive(Clone,Debug,Hash,PartialEq,Serialize,Deserialize,DieselNewType)]
        pub struct $name(pub $type);
    }
);
