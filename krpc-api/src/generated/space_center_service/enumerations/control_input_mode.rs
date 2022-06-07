/*
	summary:
		see cref="M:SpaceCenter.Control.InputMode" 
*/
pub enum ControlInputMode {
    /*
		summary:
			Control inputs are added to the vessels current control inputs.
	*/
	Additive,
	/*
		summary:
			Control inputs (when they are non-zero) override the vessels current control inputs.
	*/
	Override,
}