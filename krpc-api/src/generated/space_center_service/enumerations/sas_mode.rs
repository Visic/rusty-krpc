/*
	summary:
		The behavior of the SAS auto-pilot. See
		see cref="M:SpaceCenter.AutoPilot.SASMode" 
*/
pub enum SasMode {
    /*
		summary:
			Stability assist mode. Dampen out any rotation.
	*/
	StabilityAssist,
	/*
		summary:
			Point in the burn direction of the next maneuver node.
	*/
	Maneuver,
	/*
		summary:
			Point in the prograde direction.
	*/
	Prograde,
	/*
		summary:
			Point in the retrograde direction.
	*/
	Retrograde,
	/*
		summary:
			Point in the orbit normal direction.
	*/
	Normal,
	/*
		summary:
			Point in the orbit anti-normal direction.
	*/
	AntiNormal,
	/*
		summary:
			Point in the orbit radial direction.
	*/
	Radial,
	/*
		summary:
			Point in the orbit anti-radial direction.
	*/
	AntiRadial,
	/*
		summary:
			Point in the direction of the current target.
	*/
	Target,
	/*
		summary:
			Point away from the current target.
	*/
	AntiTarget,
}