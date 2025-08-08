import { Species } from 'beegone';

/**
 * Used at the end of switch statements or if-else chains, to guarantee that all cases are handled.
 */
export const assertNever = (value: never) => {
	throw new Error(`unhandled variant: ${value}`);
};

export const speciesName = (species: Species) => species === Species.Drone
	? 'drone'
	: species === Species.Worker
		? 'worker'
		: species === Species.Nurse
			? 'nurse'
			: species === Species.Explorer
				? 'explorer'
				: species === Species.Builder
					? 'builder'
					: species === Species.Guard
						? 'guard'
						: species === Species.Queen
							? 'queen'
							: assertNever(species);
