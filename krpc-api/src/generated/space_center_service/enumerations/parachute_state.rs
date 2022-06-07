/*
	summary:
		The state of a parachute. See
		see cref="M:SpaceCenter.Parachute.State" 
*/
pub enum ParachuteState {
    /*
		summary:
			The parachute is safely tucked away inside its housing.
	*/
	Stowed,
	/*
		summary:
			The parachute is armed for deployment. (RealChutes only)
	*/
	Armed,
	/*
		summary:
			The parachute is still stowed, but ready to semi-deploy.
			(Stock parachutes only)
	*/
	Active,
	/*
		summary:
			The parachute has been deployed and is providing some drag,
			but is not fully deployed yet. (Stock parachutes only)
	*/
	SemiDeployed,
	/*
		summary:
			The parachute is fully deployed.
	*/
	Deployed,
	/*
		summary:
			The parachute has been cut.
	*/
	Cut,
}