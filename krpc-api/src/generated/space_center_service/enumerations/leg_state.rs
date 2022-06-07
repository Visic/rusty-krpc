/*
	summary:
		The state of a landing leg. See
		see cref="M:SpaceCenter.Leg.State" 
*/
pub enum LegState {
    /*
		summary:
			Landing leg is fully deployed.
	*/
	Deployed,
	/*
		summary:
			Landing leg is fully retracted.
	*/
	Retracted,
	/*
		summary:
			Landing leg is being deployed.
	*/
	Deploying,
	/*
		summary:
			Landing leg is being retracted.
	*/
	Retracting,
	/*
		summary:
			Landing leg is broken.
	*/
	Broken,
}