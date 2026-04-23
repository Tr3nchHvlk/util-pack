pub trait Remove<Payload> {
    type OutType;
    type ErrType;
    fn remove(self, payload: Payload) -> Result<Self::OutType, Self::ErrType>;
}

pub trait RemoveAsync<Payload> {
    type OutType;
    type ErrType;
    fn remove_async(self, payload: Payload) -> impl Future<Output=Result<Self::OutType, Self::ErrType>>;
}