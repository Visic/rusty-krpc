/*
	summary:
		The state of a resource harvester. See
		see cref="M:SpaceCenter.ResourceHarvester.State" 
*/
pub enum ResourceHarvesterState {
    /*
		summary:
			The drill is deploying.
	*/
	Deploying,
	/*
		summary:
			The drill is deployed and ready.
	*/
	Deployed,
	/*
		summary:
			The drill is retracting.
	*/
	Retracting,
	/*
		summary:
			The drill is retracted.
	*/
	Retracted,
	/*
		summary:
			The drill is running.
	*/
	Active,
}