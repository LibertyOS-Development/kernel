use core::{convert::From, fmt, result};

pub type Result<T> = result::Result<T, Err>;

pub struct Err
{
	repr: Repr,
}

impl crate::err::Err for Err {}
impl fmt::Debug for Err
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		fmt::Debug::fmt(&self.repr, f)
	}
}

enum Repr
{
	Simple(ErrorKind),
	Custom(Custom),
}

#[derive(Debug)]
struct Custom
{
	kind: ErrType,
	error: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum ErrType
{
	NotFound,
	PermissionDenied,
	RefuseConnection,
	ResetConnection,
	AbortConnection,
	Disconnected,
	AddressConflict,
	UnavailableAddress,
	PipeBroken,
	Exists,
	WouldBlock,
	InputInvalid,
	DataInvalid,
	TimeOut,
	WriteZero,
	Interrupted,
	Other,
	UnexpectedEOF,
	#[doc(hidden)]
	Uncategorized,
}

impl ErrType
{
	pub(crate) fn asstr(&self) -> &'static str
	{
		match *self
		{
			ErrType::NotFound => "[ERR] NOT FOUND",
			ErrType::PermissionDenied => "[ERR] PERMISSION DENIED",
