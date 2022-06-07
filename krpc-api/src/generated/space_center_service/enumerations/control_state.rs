/*
	summary:
		The control state of a vessel.
		see cref="M:SpaceCenter.Control.State" 
*/
pub enum ControlState {
    /*
		summary:
			Full controllable.
	*/
	Full,
	/*
		summary:
			Partially controllable.
	*/
	Partial,
	/*
		summary:
			Not controllable.
	*/
	None,
}