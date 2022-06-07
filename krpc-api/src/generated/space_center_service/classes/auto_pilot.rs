/*
	summary:
		Provides basic auto-piloting utilities for a vessel.
		Created by calling
		see cref="M:SpaceCenter.Vessel.AutoPilot" 
	remarks:
		If a client engages the auto-pilot and then closes its connection to the server,
		the auto-pilot will be disengaged and its target reference frame, direction and roll
		reset to default.
*/
pub struct AutoPilot;