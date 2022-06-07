/*
	summary:
		The type of object an antenna is targetting.
		see cref="M:RemoteTech.Antenna.Target" 
*/
pub enum Target {
    /*
		summary:
			The active vessel.
	*/
	ActiveVessel,
	/*
		summary:
			A celestial body.
	*/
	CelestialBody,
	/*
		summary:
			A ground station.
	*/
	GroundStation,
	/*
		summary:
			A specific vessel.
	*/
	Vessel,
	/*
		summary:
			No target.
	*/
	None,
}