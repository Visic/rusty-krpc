/*
	summary:
		The state of a resource converter. See
		see cref="M:SpaceCenter.ResourceConverter.State" 
*/
pub enum ResourceConverterState {
    /*
		summary:
			Converter is running.
	*/
	Running,
	/*
		summary:
			Converter is idle.
	*/
	Idle,
	/*
		summary:
			Converter is missing a required resource.
	*/
	MissingResource,
	/*
		summary:
			No available storage for output resource.
	*/
	StorageFull,
	/*
		summary:
			At preset resource capacity.
	*/
	Capacity,
	/*
		summary:
			Unknown state. Possible with modified resource converters.
			In this case, check
			see cref="M:SpaceCenter.ResourceConverter.StatusInfo" 
			for more information.
	*/
	Unknown,
}