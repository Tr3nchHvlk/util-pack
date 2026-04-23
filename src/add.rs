pub trait SelfAdd {
    type OkType;
    type ErrType;
    fn add(self) -> Result<Self::OkType, Self::ErrType>;
}

pub trait SelfAddAsync<Payload> {
    type OutType;
    type ErrType;
    fn add_async(payload: Payload) -> impl Future<Output=Result<Self::OutType, Self::ErrType>>;
}

pub trait Add<Payload> {
    type OutType;
    type ErrType;
    fn add(self, payload: Payload) -> Result<Self::OutType, Self::ErrType>;
}

pub trait AddAsync<Payload> {
    type OutType;
    type ErrType;
    fn add_async(self, payload: Payload) -> impl Future<Output=Result<Self::OutType, Self::ErrType>>;
}