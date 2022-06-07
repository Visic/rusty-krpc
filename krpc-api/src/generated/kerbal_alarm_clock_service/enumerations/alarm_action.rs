/*
	summary:
		The action performed by an alarm when it fires.
*/
pub enum AlarmAction {
    /*
		summary:
			Don't do anything at all...
	*/
	DoNothing,
	/*
		summary:
			Don't do anything, and delete the alarm.
	*/
	DoNothingDeleteWhenPassed,
	/*
		summary:
			Drop out of time warp.
	*/
	KillWarp,
	/*
		summary:
			Drop out of time warp.
	*/
	KillWarpOnly,
	/*
		summary:
			Display a message.
	*/
	MessageOnly,
	/*
		summary:
			Pause the game.
	*/
	PauseGame,
}