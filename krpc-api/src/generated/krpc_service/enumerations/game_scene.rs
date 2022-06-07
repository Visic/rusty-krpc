/*
	summary:
		The game scene. See
		see cref="M:KRPC.CurrentGameScene" 
*/
pub enum GameScene {
    /*
		summary:
			The game scene showing the Kerbal Space Center buildings.
	*/
	SpaceCenter,
	/*
		summary:
			The game scene showing a vessel in flight (or on the launchpad/runway).
	*/
	Flight,
	/*
		summary:
			The tracking station.
	*/
	TrackingStation,
	/*
		summary:
			The Vehicle Assembly Building.
	*/
	EditorVAB,
	/*
		summary:
			The Space Plane Hangar.
	*/
	EditorSPH,
}