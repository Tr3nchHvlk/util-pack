// usage: impl StreamedGet<GetStream<PayloadType>> for System where PayloadType: IntoStreamHandler

pub trait StreamHandler<Item> {
    type ErrType;
    fn next(&mut self) -> Result<Option<Item>, Self::ErrType>;
}

pub trait AsyncStreamHandler<Item> {
    type ErrType;
    fn next(&mut self) -> impl Future<Output=Result<Option<Item>, Self::ErrType>>;
}

pub trait GetStreamHandler<StreamedItem> {
    type PayloadType;
    type ErrType;
    fn get(payload: Self::PayloadType) -> impl StreamHandler<StreamedItem>;
}

pub trait GetAsyncStreamHandler<StreamedItem> {
    type PayloadType;
    type ErrType;
    fn get(payload: Self::PayloadType) -> impl AsyncStreamHandler<StreamedItem>;
}

pub trait IntoStreamHandler where Self: Sized {
    type PayloadType;
    type ErrType;

    fn into_stream_handler(payload: Self::PayloadType) -> Result<impl StreamHandler<Self>, Self::ErrType>;
    // impl StreamHandler<Self>;
}

pub trait IntoStreamHandlerAsync where Self: Sized {
    type PayloadType;
    type ErrType;

    fn into_stream_handler_async(payload: Self::PayloadType) -> Result<impl AsyncStreamHandler<Self>, Self::ErrType>;
}

pub struct GetStream<PayloadType> {
    pub payload: PayloadType,
}

pub struct GetStreamAsync<PayloadType> {
    pub payload: PayloadType,
}