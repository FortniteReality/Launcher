export const getGameStatusBadge = (
	installed: boolean,
	isInstalling: boolean
): { className: string; text: string } => {
	if (installed) {
		return {
			className: "bg-green-500/20 text-green-300 border-green-500/30",
			text: "Installed"
		}
	}
	if (isInstalling) {
		return {
			className: "bg-blue-500/20 text-blue-300 border-blue-500/30",
			text: "Installing"
		}
	}
	return {
		className: "bg-gray-500/20 text-gray-300 border-gray-500/30",
		text: "Not Installed"
	}
}

export const formatInstallLocation = (location: string, maxLength: number = 32): string => {
	if (location.length <= maxLength) return location
	return `...${location.slice(-(maxLength - 3))}`
}