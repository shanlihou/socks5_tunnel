pub type CommonErr = Box<dyn std::error::Error + Send + Sync>;
pub type CommonRet<T> = Result<T, CommonErr>;

