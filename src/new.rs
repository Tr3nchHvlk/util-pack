pub trait SelfInit<PayloadType> where Self: Sized {
    type ErrType;
    fn init(payload: PayloadType) -> Result<Self, Self::ErrType>;
}

pub trait SelfInitAsync<PayloadType> where Self: Sized {
    type ErrType;
    fn init_async(payload: PayloadType) -> impl Future<Output=Result<Self, Self::ErrType>>;
}

// pub trait GetNew<PayloadType> where Self: Sized {
//     type ErrType;
//     fn get(payload: PayloadType) -> Result<Self, Self::ErrType>;
// }

// pub trait GetNewAsync<PayloadType> where Self: Sized {
//     type ErrType;
//     fn get_async(payload: PayloadType) -> impl Future<Output=Result<Self, Self::ErrType>>;
// }