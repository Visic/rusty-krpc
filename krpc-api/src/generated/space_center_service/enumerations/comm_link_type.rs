/*
	summary:
		The type of a communication link.
		see cref="M:SpaceCenter.CommLink.Type" 
*/
pub enum CommLinkType {
    /*
		summary:
			Link is to a base station on Kerbin.
	*/
	Home,
	/*
		summary:
			Link is to a control source, for example a manned spacecraft.
	*/
	Control,
	/*
		summary:
			Link is to a relay satellite.
	*/
	Relay,
}