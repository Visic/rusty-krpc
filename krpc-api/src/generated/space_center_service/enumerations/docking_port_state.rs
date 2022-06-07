/*
	summary:
		The state of a docking port. See
		see cref="M:SpaceCenter.DockingPort.State" 
*/
pub enum DockingPortState {
    /*
		summary:
			The docking port is ready to dock to another docking port.
	*/
	Ready,
	/*
		summary:
			The docking port is docked to another docking port, or docked to
			another part (from the VAB/SPH).
	*/
	Docked,
	/*
		summary:
			The docking port is very close to another docking port,
			but has not docked. It is using magnetic force to acquire a solid dock.
	*/
	Docking,
	/*
		summary:
			The docking port has just been undocked from another docking port,
			and is disabled until it moves away by a sufficient distance
			(
			see cref="M:SpaceCenter.DockingPort.ReengageDistance" 
			).
	*/
	Undocking,
	/*
		summary:
			The docking port has a shield, and the shield is closed.
	*/
	Shielded,
	/*
		summary:
			The docking ports shield is currently opening/closing.
	*/
	Moving,
}