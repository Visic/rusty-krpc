/*
	summary:
		The control source of a vessel.
		see cref="M:SpaceCenter.Control.Source" 
*/
pub enum ControlSource {
    /*
		summary:
			Vessel is controlled by a Kerbal.
	*/
	Kerbal,
	/*
		summary:
			Vessel is controlled by a probe core.
	*/
	Probe,
	/*
		summary:
			Vessel is not controlled.
	*/
	None,
}