/*
	summary:
		The time warp mode.
		Returned by
		see cref="T:SpaceCenter.WarpMode" 
*/
pub enum WarpMode {
    /*
		summary:
			Time warp is active, and in regular "on-rails" mode.
	*/
	Rails,
	/*
		summary:
			Time warp is active, and in physical time warp mode.
	*/
	Physics,
	/*
		summary:
			Time warp is not active.
	*/
	None,
}