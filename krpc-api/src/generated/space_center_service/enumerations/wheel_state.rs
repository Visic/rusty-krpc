/*
	summary:
		The state of a wheel. See
		see cref="M:SpaceCenter.Wheel.State" 
*/
pub enum WheelState {
    /*
		summary:
			Wheel is fully deployed.
	*/
	Deployed,
	/*
		summary:
			Wheel is fully retracted.
	*/
	Retracted,
	/*
		summary:
			Wheel is being deployed.
	*/
	Deploying,
	/*
		summary:
			Wheel is being retracted.
	*/
	Retracting,
	/*
		summary:
			Wheel is broken.
	*/
	Broken,
}