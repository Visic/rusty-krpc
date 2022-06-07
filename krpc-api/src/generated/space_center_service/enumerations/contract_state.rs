/*
	summary:
		The state of a contract. See
		see cref="M:SpaceCenter.Contract.State" 
*/
pub enum ContractState {
    /*
		summary:
			The contract is active.
	*/
	Active,
	/*
		summary:
			The contract has been canceled.
	*/
	Canceled,
	/*
		summary:
			The contract has been completed.
	*/
	Completed,
	/*
		summary:
			The deadline for the contract has expired.
	*/
	DeadlineExpired,
	/*
		summary:
			The contract has been declined.
	*/
	Declined,
	/*
		summary:
			The contract has been failed.
	*/
	Failed,
	/*
		summary:
			The contract has been generated.
	*/
	Generated,
	/*
		summary:
			The contract has been offered to the player.
	*/
	Offered,
	/*
		summary:
			The contract was offered to the player, but the offer expired.
	*/
	OfferExpired,
	/*
		summary:
			The contract has been withdrawn.
	*/
	Withdrawn,
}