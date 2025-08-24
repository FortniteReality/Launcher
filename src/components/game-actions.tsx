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

const formatTimeRemaining = (seconds: number) => {
	if (seconds < 60) {
		return `${Math.round(seconds)}s`
	} else if (seconds < 3600) {
		const mins = Math.floor(seconds / 60)
		const secs = Math.round(seconds % 60)
		return `${mins}m ${secs}s`
	} else {
		const hours = Math.floor(seconds / 3600)
		const mins = Math.floor((seconds % 3600) / 60)
		return mins > 0 ? `${hours}h ${mins}m` : `${hours}h`
	}
}

const formatDownloadSpeed = (bytesPerSecond: number) => {
	if (bytesPerSecond < 1024) {
		return `${bytesPerSecond.toFixed(2)} B/s`
	} else if (bytesPerSecond < 1048576) {
		return `${(bytesPerSecond / 1024).toFixed(2)} KB/s`
	} else if (bytesPerSecond < 1073741824) {
		return `${(bytesPerSecond / 1048576).toFixed(2)} MB/s`
	} else {
		return `${(bytesPerSecond / 1073741824).toFixed(2)} GB/s`
	}
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

					<div className="flex justify-between text-xs text-gray-400 mb-2">
						<span>
							{installState.downloadSpeed && installState.downloadSpeed > 0
								? formatDownloadSpeed(installState.downloadSpeed)
								: "Calculating speed..."
							}
						</span>
						<span>
							{installState.eta && installState.eta > 0
								? `ETA: ${formatTimeRemaining(installState.eta)}`
								: "Calculating time..."
							}
						</span>
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