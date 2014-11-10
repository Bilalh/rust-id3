#![macro_escape]

extern crate audiotag;

macro_rules! try_delim {
    ($enc:expr, $data:expr, $i:expr, $msg:expr) => {
        match util::find_delim($enc, $data, $i) {
            Some(i) => i,
            None => return Err(TagError::new(audiotag::InvalidInputError, $msg))
        }
    };
}

macro_rules! try_encoding {
    ($c:expr) => {
        {
            let encoding: encoding::Encoding = match FromPrimitive::from_u8($c) {
                Some(encoding) => encoding,
                None => return Err(TagError::new(audiotag::InvalidInputError, "invalid encoding byte"))
            };
            encoding
        }
    };
}

macro_rules! try_string {
    ($enc:expr, $data:expr) => {
        match util::string_from_encoding($enc, $data) {
            Ok(string) => string,
            Err(bytes) => return Err(TagError::new(audiotag::StringDecodingError(bytes), match $enc {
                ::frame::encoding::Latin1 | ::frame::encoding::UTF8 => "string is not valid utf8",
                ::frame::encoding::UTF16 => "string is not valid utf16",
                ::frame::encoding::UTF16BE => "string is not valid utf16-be"
            }))
        }
    };
}