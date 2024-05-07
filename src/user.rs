pub mod register {
    type Result = core::result::Result<(), RegErr>;

    #[derive(Debug)]
    pub enum RegErr {
        _Unknown,
    }
    pub async fn register() -> Result {
        unimplemented!("注册的具体操作");
    }
}
