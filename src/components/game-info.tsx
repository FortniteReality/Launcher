import React from "react"
import { Badge } from "./ui/badge"
import { Game } from "../types/game"
import { getGameStatusBadge, formatInstallLocation } from "../utils/game-utils"

interface GameInfoProps {
	game: Game
	gameInstalled: boolean
	isInstalling: boolean
	installLocation: string
}

export const GameInfo: React.FC<GameInfoProps> = ({
	game,
	gameInstalled,
	isInstalling,
	installLocation
}) => {
	const statusBadge = getGameStatusBadge(gameInstalled, isInstalling)

	return (
		<div className="space-y-3">
			<div className="flex justify-between">
				<span className="text-gray-400">Version</span>
				<span className="text-gray-100">{game.version.substring(game.version.lastIndexOf('+') + 1).split('-', 2).join(" ")}</span>
			</div>
			<div className="flex justify-between">
				<span className="text-gray-400">Status</span>
				<Badge className={statusBadge.className}>
					{statusBadge.text}
				</Badge>
			</div>
			{gameInstalled && (
				<div className="flex justify-between">
					<span className="text-gray-400">Location</span>
					<span 
						className="text-gray-100 text-xs truncate max-w-32" 
						title={installLocation}
					>
						{formatInstallLocation(installLocation)}
					</span>
				</div>
			)}
		</div>
	)
}