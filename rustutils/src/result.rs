use std::fmt::Debug;

pub trait OkResultIfExt<OkHandler, OkVal, ErrVal>
where
    OkHandler: Fn(&OkVal),
    ErrVal: Debug,
{
    fn if_ok(self, handler: OkHandler) -> Self;
}

impl<OkHandler, OkVal, ErrVal> OkResultIfExt<OkHandler, OkVal, ErrVal> for Result<OkVal, ErrVal>
where
    OkHandler: Fn(&OkVal),
    ErrVal: Debug,
{
    fn if_ok(self, handler: OkHandler) -> Self {
        match self {
            Ok(value) => {
                handler(&value);
                Ok(value)
            }
            Err(_) => self,
        }
    }
}

pub trait ErrResultIfExt<ErrHandler, OkVal, ErrVal>
where
    ErrHandler: Fn(&ErrVal),
    ErrVal: Debug,
{
    fn if_err(self, handler: ErrHandler) -> Self;
}

impl<ErrHandler, OkVal, ErrVal> ErrResultIfExt<ErrHandler, OkVal, ErrVal> for Result<OkVal, ErrVal>
where
    ErrHandler: Fn(&ErrVal),
    ErrVal: Debug,
{
    fn if_err(self, handler: ErrHandler) -> Self {
        match self {
            Ok(_) => self,
            Err(value) => {
                handler(&value);
                Err(value)
            }
        }
    }
}
