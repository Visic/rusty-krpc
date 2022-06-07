/*
	summary:
		see cref="M:SpaceCenter.Camera.Mode" 
*/
pub enum CameraMode {
    /*
		summary:
			The camera is showing the active vessel, in "auto" mode.
	*/
	Automatic,
	/*
		summary:
			The camera is showing the active vessel, in "free" mode.
	*/
	Free,
	/*
		summary:
			The camera is showing the active vessel, in "chase" mode.
	*/
	Chase,
	/*
		summary:
			The camera is showing the active vessel, in "locked" mode.
	*/
	Locked,
	/*
		summary:
			The camera is showing the active vessel, in "orbital" mode.
	*/
	Orbital,
	/*
		summary:
			The Intra-Vehicular Activity view is being shown.
	*/
	IVA,
	/*
		summary:
			The map view is being shown.
	*/
	Map,
}