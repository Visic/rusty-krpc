/*
	summary:
		The situation a vessel is in.
		see cref="M:SpaceCenter.Vessel.Situation" 
*/
pub enum VesselSituation {
    /*
		summary:
			Vessel is awaiting launch.
	*/
	PreLaunch,
	/*
		summary:
			Vessel is orbiting a body.
	*/
	Orbiting,
	/*
		summary:
			Vessel is on a sub-orbital trajectory.
	*/
	SubOrbital,
	/*
		summary:
			Escaping.
	*/
	Escaping,
	/*
		summary:
			Vessel is flying through an atmosphere.
	*/
	Flying,
	/*
		summary:
			Vessel is landed on the surface of a body.
	*/
	Landed,
	/*
		summary:
			Vessel has splashed down in an ocean.
	*/
	Splashed,
	/*
		summary:
			Vessel is docked to another.
	*/
	Docked,
}