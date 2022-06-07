/*
	summary:
		The state of a radiator.
		see cref="T:SpaceCenter.RadiatorState" 
*/
pub enum RadiatorState {
    /*
		summary:
			Radiator is fully extended.
	*/
	Extended,
	/*
		summary:
			Radiator is fully retracted.
	*/
	Retracted,
	/*
		summary:
			Radiator is being extended.
	*/
	Extending,
	/*
		summary:
			Radiator is being retracted.
	*/
	Retracting,
	/*
		summary:
			Radiator is being broken.
	*/
	Broken,
}