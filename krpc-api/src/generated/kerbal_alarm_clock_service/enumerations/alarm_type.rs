/*
	summary:
		The type of an alarm.
*/
pub enum AlarmType {
    /*
		summary:
			An alarm for a specific date/time or a specific period in the future.
	*/
	Raw,
	/*
		summary:
			An alarm based on the next maneuver node on the current ships flight path.
			This node will be stored and can be restored when you come back to the ship.
	*/
	Maneuver,
	/*
		summary:
			see cref="M:KerbalAlarmClock.AlarmType.Maneuver" 
	*/
	ManeuverAuto,
	/*
		summary:
			An alarm for furthest part of the orbit from the planet.
	*/
	Apoapsis,
	/*
		summary:
			An alarm for nearest part of the orbit from the planet.
	*/
	Periapsis,
	/*
		summary:
			Ascending node for the targeted object, or equatorial ascending node.
	*/
	AscendingNode,
	/*
		summary:
			Descending node for the targeted object, or equatorial descending node.
	*/
	DescendingNode,
	/*
		summary:
			An alarm based on the closest approach of this vessel to the targeted
			vessel, some number of orbits into the future.
	*/
	Closest,
	/*
		summary:
			An alarm based on the expiry or deadline of contracts in career modes.
	*/
	Contract,
	/*
		summary:
			see cref="M:KerbalAlarmClock.AlarmType.Contract" 
	*/
	ContractAuto,
	/*
		summary:
			An alarm that is attached to a crew member.
	*/
	Crew,
	/*
		summary:
			An alarm that is triggered when a selected target comes within a chosen distance.
	*/
	Distance,
	/*
		summary:
			An alarm based on the time in the "Earth" alternative Universe (aka the Real World).
	*/
	EarthTime,
	/*
		summary:
			An alarm that fires as your landed craft passes under the orbit of your target.
	*/
	LaunchRendevous,
	/*
		summary:
			An alarm manually based on when the next SOI point is on the flight path
			or set to continually monitor the active flight path and add alarms as it
			detects SOI changes.
	*/
	SOIChange,
	/*
		summary:
			see cref="M:KerbalAlarmClock.AlarmType.SOIChange" 
	*/
	SOIChangeAuto,
	/*
		summary:
			An alarm based on Interplanetary Transfer Phase Angles, i.e. when should
			I launch to planet X? Based on Kosmo Not's post and used in Olex's
			Calculator.
	*/
	Transfer,
	/*
		summary:
			see cref="M:KerbalAlarmClock.AlarmType.Transfer" 
	*/
	TransferModelled,
}