/*
	summary:
		The state of a solar panel. See
		see cref="M:SpaceCenter.SolarPanel.State" 
*/
pub enum SolarPanelState {
    /*
		summary:
			Solar panel is fully extended.
	*/
	Extended,
	/*
		summary:
			Solar panel is fully retracted.
	*/
	Retracted,
	/*
		summary:
			Solar panel is being extended.
	*/
	Extending,
	/*
		summary:
			Solar panel is being retracted.
	*/
	Retracting,
	/*
		summary:
			Solar panel is broken.
	*/
	Broken,
}