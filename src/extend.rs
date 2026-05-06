pub trait ExtendSelf<PayloadType> {
    fn extend_self(self, payload: PayloadType) -> Self;
}