/*
	summary:
		The way in which a resource flows between parts. See
		see cref="M:SpaceCenter.Resources.FlowMode" 
*/
pub enum ResourceFlowMode {
    /*
		summary:
			The resource flows to any part in the vessel. For example, electric charge.
	*/
	Vessel,
	/*
		summary:
			The resource flows from parts in the first stage, followed by the second,
			and so on. For example, mono-propellant.
	*/
	Stage,
	/*
		summary:
			The resource flows between adjacent parts within the vessel. For example,
			liquid fuel or oxidizer.
	*/
	Adjacent,
	/*
		summary:
			The resource does not flow. For example, solid fuel.
	*/
	None,
}