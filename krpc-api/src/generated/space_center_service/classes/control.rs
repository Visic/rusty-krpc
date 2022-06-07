/*
	summary:
		Used to manipulate the controls of a vessel. This includes adjusting the
		throttle, enabling/disabling systems such as SAS and RCS, or altering the
		direction in which the vessel is pointing.
		Obtained by calling
		see cref="M:SpaceCenter.Vessel.Control" 
	remarks:
		Control inputs (such as pitch, yaw and roll) are zeroed when all clients
		that have set one or more of these inputs are no longer connected.
*/
pub struct Control;