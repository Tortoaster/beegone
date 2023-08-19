export function polygon(viewBox: number, radius: number, cornerRadius = 0, sides = 6): string {
	const centerX = viewBox / 2;
	const centerY = viewBox / 2;

	// The angle between the corner of the polygon and the point where it intersects the arc,
	// relative to the center of the arc
	const arcCenterAngle = Math.PI / 2 - ((Math.PI / 2) * (sides - 2)) / sides;
	// The vertical distance between the centerpoint of the arc and the intersection point
	const y = Math.sin(arcCenterAngle) * cornerRadius;
	// The horizontal distance between the centerpoint of the arc and the intersection point
	const dx = Math.cos(arcCenterAngle) * cornerRadius;
	// The distance between the centerpoint of an arc and the center of the polygon
	const x = radius - cornerRadius / Math.cos(arcCenterAngle) + dx;
	// Then angle between the corner of the polygon and the point where it intersects the arc,
	// relative to the center of the polygon
	const arcAngle = Math.atan2(y, x);
	const arcRadius = Math.hypot(x, y);

	const arcEndX = centerX + Math.cos(arcAngle) * arcRadius;
	const arcEndY = centerY + Math.sin(arcAngle) * arcRadius;

	let path = `M${arcEndX},${arcEndY}`;

	for (let a = 1; a <= sides; a++) {
		const angle = a * ((2 * Math.PI) / sides);
		const startAngle = angle - arcAngle;
		const endAngle = angle + arcAngle;
		const arcStartX = centerX + Math.cos(startAngle) * arcRadius;
		const arcStartY = centerY + Math.sin(startAngle) * arcRadius;
		const arcEndX = centerX + Math.cos(endAngle) * arcRadius;
		const arcEndY = centerY + Math.sin(endAngle) * arcRadius;
		path += `L${arcStartX},${arcStartY}`;
		path += `A${cornerRadius},${cornerRadius},0,0,1,${arcEndX},${arcEndY}`;
	}

	return path + 'Z';
}
