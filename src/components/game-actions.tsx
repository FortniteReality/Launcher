import React from "react"
import { Button } from "./ui/button"
import { Download, Play, Pause, X } from "lucide-react"
import { InstallState } from "../types/launcher"

interface GameActionsProps {
	gameInstalled: boolean
	isPlaying: boolean
	installState: InstallState
	onLaunchGame: () => void
	onInstallGame: () => void
	onCancelInstall: () => void
}

export const GameActions: React.FC<GameActionsProps> = ({
	gameInstalled,
	isPlaying,
	installState,
	onLaunchGame,
	onInstallGame,
	onCancelInstall,
}) => {
	if (gameInstalled) {
		return (
			<div className="flex flex-col items-end gap-3">
				<Button
					size="lg"
					className="bg-green-600 hover:bg-green-700 text-white px-8 py-3 text-lg"
					onClick={onLaunchGame}
					disabled={isPlaying}
				>
					{isPlaying ? (
						<>
							<Pause className="h-5 w-5 mr-2" />
							Running
						</>
					) : (
						<>
							<Play className="h-5 w-5 mr-2" />
							Play
						</>
					)}
				</Button>
			</div>
		)
	}

	return (
		<div className="flex flex-col items-end gap-2">
			<div className="flex items-center gap-2">
				<Button
					size="lg"
					className="bg-blue-600 hover:bg-blue-700 text-white px-8 py-3 text-lg"
					onClick={onInstallGame}
					disabled={installState.isInstalling}
				>
					<Download className="h-5 w-5 mr-2" />
					{installState.isInstalling ? "Installing..." : "Install"}
				</Button>

				{installState.isInstalling && (
					<Button
						size="lg"
						variant="outline"
						className="border-red-600 text-red-400 hover:bg-red-600/10 px-4 py-3 bg-transparent"
						onClick={onCancelInstall}
					>
						<X className="h-5 w-5" />
					</Button>
				)}
			</div>

			{installState.isInstalling && (
				<div className="w-64">
					<div className="flex justify-between text-sm text-gray-300 mb-1">
						<span>Installing to {installState.location}</span>
						<span>{installState.progress.toFixed(2)}%</span>
					</div>
					<progress 
						value={installState.progress} 
						max="100"
						className="w-full h-2 rounded-full bg-gray-800 [&::-webkit-progress-bar]:bg-gray-800 [&::-webkit-progress-bar]:rounded-full [&::-webkit-progress-value]:bg-blue-500 [&::-webkit-progress-value]:rounded-full [&::-moz-progress-bar]:bg-blue-500 [&::-moz-progress-bar]:rounded-full"
					/>
				</div>
			)}
		</div>
	)
}