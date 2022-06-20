/// Comodity wrapper for Result<T, crate::Error>
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
/// Enumeration of the possible errors that might ariase in this crate
pub enum Error {
    /// This error is returned when the backend could not write a word of memory
    /// to the stream
    WriteFailed,
    /// This error is returned when the backend could not write a bit
    /// to the stream
    WriteBitFailed,
    /// This error is returned when the backend could not read a word of memory
    /// from the stream
    ReadFailed(String),
    /// This error is returned when the backend could not read a bit
    /// from the stream
    ReadBitFailed,
    /// This error is returned when the Interpolative encoding routine is called
    /// with an empty array.
    InterpolativeCodeWithEmptyArray,

    /// This errors is returned when trying to convert a nibble to a code
    InvalidCodeNibble(u8),

    /// This error is returned when the opening of a file failed. The inner value
    /// is the path
    UnableToOpenFile(String),
    OpenError(std::io::Error),

    /// This error is returned when an mmap fails
    MMapError(String),

    /// This error is raised when there was a missing property on a webgraph 
    /// propeties file
    MissingPropertiyError{
        key: String,
    },

    /// This error is raised when there was property in a webgraph 
    /// propeties file cannot be parsed with the appropriate type
    PropertiyParsingError{
        key: String,
        _type: String,
        value: String,
    },
}
