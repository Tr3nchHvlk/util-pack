pub trait GetExplicit<Target, PayloadType> {
    type ErrType;
    fn get(self, payload: PayloadType) -> Result<Target, Self::ErrType>;
}

pub trait GetAsyncExplicit<Target, PayloadType> {
    type ErrType;
    fn get_async(self, payload: PayloadType) -> impl Future<Output=Result<Target, Self::ErrType>>;
}

pub trait SelfGet where Self: Sized {
    type PayloadType;
    type ErrType;
    fn get(payload: Self::PayloadType) -> Result<Self, Self::ErrType>;
}

pub trait SelfGetAsync where Self: Sized {
    type PayloadType;
    type ErrType;
    fn get(payload: Self::PayloadType) -> impl Future<Output=Result<Self, Self::ErrType>>;
}

pub trait Get<Target> {
    type PayloadType;
    type ErrType;
    fn get(self, payload: Self::PayloadType) -> Result<Target, Self::ErrType>;
}

pub trait GetAsync<Target> {
    type PayloadType;
    type ErrType;
    fn get(self, payload: Self::PayloadType) -> impl Future<Output=Result<Target, Self::ErrType>>;
}