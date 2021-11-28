struct Log;

impl log::Log for Log
{
	fn active(&self, metadata: &log::Metadata) -> bool
	{
		if cfg!(debug_assertions)
		{
			true
		}
	else
	{
		metadata.level() <= log::Level::Info
