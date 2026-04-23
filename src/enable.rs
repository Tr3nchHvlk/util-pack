pub trait EnableAsync<Payload> {
    type StatusOutType;
    type ErrType;
    fn enable_async(self, payload: Payload) -> impl Future<Output=Result<Self::StatusOutType, Self::ErrType>>;
}