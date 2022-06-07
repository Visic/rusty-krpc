/*
	summary:
		The game mode.
		Returned by
		see cref="T:SpaceCenter.GameMode" 
*/
pub enum GameMode {
    /*
		summary:
			Sandbox mode.
	*/
	Sandbox,
	/*
		summary:
			Career mode.
	*/
	Career,
	/*
		summary:
			Science career mode.
	*/
	Science,
	/*
		summary:
			Science sandbox mode.
	*/
	ScienceSandbox,
	/*
		summary:
			Mission mode.
	*/
	Mission,
	/*
		summary:
			Mission builder mode.
	*/
	MissionBuilder,
	/*
		summary:
			Scenario mode.
	*/
	Scenario,
	/*
		summary:
			Scenario mode that cannot be resumed.
	*/
	ScenarioNonResumable,
}