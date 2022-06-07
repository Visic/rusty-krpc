/*
	summary:
		The type of a crew member.
		see cref="M:SpaceCenter.CrewMember.Type" 
*/
pub enum CrewMemberType {
    /*
		summary:
			An applicant for crew.
	*/
	Applicant,
	/*
		summary:
			Rocket crew.
	*/
	Crew,
	/*
		summary:
			A tourist.
	*/
	Tourist,
	/*
		summary:
			An unowned crew member.
	*/
	Unowned,
}