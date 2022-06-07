/*
	summary:
		The state of a cargo bay. See
		see cref="M:SpaceCenter.CargoBay.State" 
*/
pub enum CargoBayState {
    /*
		summary:
			Cargo bay is fully open.
	*/
	Open,
	/*
		summary:
			Cargo bay closed and locked.
	*/
	Closed,
	/*
		summary:
			Cargo bay is opening.
	*/
	Opening,
	/*
		summary:
			Cargo bay is closing.
	*/
	Closing,
}