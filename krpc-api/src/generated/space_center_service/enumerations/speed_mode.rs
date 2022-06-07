/*
	summary:
		The mode of the speed reported in the navball.
		see cref="M:SpaceCenter.Control.SpeedMode" 
*/
pub enum SpeedMode {
    /*
		summary:
			Speed is relative to the vessel's orbit.
	*/
	Orbit,
	/*
		summary:
			Speed is relative to the surface of the body being orbited.
	*/
	Surface,
	/*
		summary:
			Speed is relative to the current target.
	*/
	Target,
}