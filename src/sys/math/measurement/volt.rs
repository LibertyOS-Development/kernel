// src/math/measurement/volt.rs
//
// Provides the kernel with the ability to work with voltage.


#[derive(Copy, Clone, Debug)]
pub struct Volt
{
	volts: f64,
}

impl Volt
{
	// New Volt, from volts.
	pub fn from_volt(volts: f64) -> Self
	{
		Volt
		{
			volts
		}
	}

	// New Volt, from microvolts.
	pub fn from_microvolt(microvolts: f64) -> Self
	{
		// Convert value from volts to microvolts
		Self::from_volt(microvolts / 1_000_000.0)
	}

	// New Volt, from millivolts.
	pub fn from_millivolt(millivolts: f64) -> Self
	{
		// Convert value from volts to millivolts
		Self::from_volt(millivolts / 1000.0)
	}

	// New Volt, from kilovolts.
	pub fn from_kvolt(kvolts: f64) -> Self
	{
		// Convert value from volts to kilovolts
		Self::from_volt(kvolts * 1000.0)
	}

	// Convert volt into floating-point (volts).
	pub fn to_volt(&self) -> f64
	{
		self.volts
	}

	// Convert volt into floating-point (microvolts).
	pub fn to_microvolt(&self) -> f64
	{
		// Convert volts into microvolts
		self.volts * 1_000_000.0
	}

	// Convert volt into floating-point (millivolts).
	pub fn to_millivolt(&self) -> f64
	{
		// Convert volts into millivolts
		self.volts * 1000.0
	}

	// Convert volt into floating-point (kilovolts).
	pub fn to_kvolt(&self) -> f64
	{
		// Convert volts to kilovolts
		self.volts / 1000.0
	}
}

