/*
	summary:
		The state of an antenna. See
		see cref="M:SpaceCenter.Antenna.State" 
*/
pub enum AntennaState {
    /*
		summary:
			Antenna is fully deployed.
	*/
	Deployed,
	/*
		summary:
			Antenna is fully retracted.
	*/
	Retracted,
	/*
		summary:
			Antenna is being deployed.
	*/
	Deploying,
	/*
		summary:
			Antenna is being retracted.
	*/
	Retracting,
	/*
		summary:
			Antenna is broken.
	*/
	Broken,
}