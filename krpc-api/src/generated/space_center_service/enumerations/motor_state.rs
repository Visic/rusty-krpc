/*
	summary:
		The state of the motor on a powered wheel. See
		see cref="M:SpaceCenter.Wheel.MotorState" 
*/
pub enum MotorState {
    /*
		summary:
			The motor is idle.
	*/
	Idle,
	/*
		summary:
			The motor is running.
	*/
	Running,
	/*
		summary:
			The motor is disabled.
	*/
	Disabled,
	/*
		summary:
			The motor is inoperable.
	*/
	Inoperable,
	/*
		summary:
			The motor does not have enough resources to run.
	*/
	NotEnoughResources,
}