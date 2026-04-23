pub trait Insert<Payload> {
    type OutType;
    type ErrType;
    fn insert(self, payload: Payload) -> Result<Self::OutType, Self::ErrType>;
}

pub trait InsertAsync<Payload> {
    type OutType;
    type ErrType;
    fn insert_async(self, payload: Payload) -> impl Future<Output=Result<Self::OutType, Self::ErrType>>;
}