export enum Color {
	Light,
	Dark,
}

export enum Species {
	Drone,
	Worker,
	Nurse,
	Explorer,
	Builder,
	Guard,
	Queen,
}

export type Piece =
	| { type: 'Bee'; content: [Color, Species] }
	| { type: 'Wall'; content: undefined };

export interface Pos {
	q: number;
	r: number;
}
