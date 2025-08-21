import React from "react"
import { Button } from "./ui/button"
import { RotateCcw, CheckCircle, AlertCircle, Trash2 } from "lucide-react"
import { UninstallState, VerifyState } from "../types/launcher"

interface QuickActionsProps {
	gameInstalled: boolean
	verifyState: VerifyState
	uninstallState: UninstallState
	onVerifyFiles: () => void
	onUninstallGame: () => void
}

export const QuickActions: React.FC<QuickActionsProps> = ({
	gameInstalled,
	verifyState,
	uninstallState,
	onVerifyFiles,
	onUninstallGame
}) => (
	<div className="space-y-3">
		<Button
			variant="outline"
			className="w-full border-gray-700 text-gray-300 hover:bg-black/20 bg-transparent relative"
			disabled={!gameInstalled || verifyState.isVerifying || uninstallState.isUninstalling}
			onClick={onVerifyFiles}
		>
			{verifyState.isVerifying ? (
				<>
					<RotateCcw className="h-4 w-4 mr-2 animate-spin" />
					Verifying... {verifyState.progress.toFixed(2)}%
				</>
			) : (
				<>
					<RotateCcw className="h-4 w-4 mr-2" />
					Verify Files
				</>
			)}

			{verifyState.result && (
				<div className="absolute -top-2 -right-2">
					{verifyState.result === "success" ? (
						<CheckCircle className="h-5 w-5 text-green-400" />
					) : (
						<AlertCircle className="h-5 w-5 text-red-400" />
					)}
				</div>
			)}
		</Button>

		<Button
			variant="outline"
			className="w-full border-red-700 text-red-300 hover:bg-red-600/20 bg-transparent relative"
			disabled={!gameInstalled || verifyState.isVerifying || uninstallState.isUninstalling}
			onClick={onUninstallGame}
		>
			{uninstallState.isUninstalling ? (
				<>
					<Trash2 className="h-4 w-4 mr-2 animate-spin" />
					Uninstalling... {Math.round(uninstallState.progress)}%
				</>
			) : (
				<>
					<Trash2 className="h-4 w-4 mr-2" />
					Uninstall Game
				</>
			)}

			{uninstallState.result && (
				<div className="absolute -top-2 -right-2">
					{uninstallState.result === "success" ? (
						<CheckCircle className="h-5 w-5 text-green-400" />
					) : (
						<AlertCircle className="h-5 w-5 text-red-400" />
					)}
				</div>
			)}
		</Button>

		{verifyState.isVerifying && (
			<div className="w-full">
				<progress 
					value={verifyState.progress} 
					max="100"
					className="w-full h-1 rounded-full bg-gray-800 [&::-webkit-progress-bar]:bg-gray-800 [&::-webkit-progress-bar]:rounded-full [&::-webkit-progress-value]:bg-blue-500 [&::-webkit-progress-value]:rounded-full [&::-moz-progress-bar]:bg-blue-500 [&::-moz-progress-bar]:rounded-full"
				/>
			</div>
		)}

		{uninstallState.isUninstalling && (
			<div className="w-full">
				<progress 
					value={uninstallState.progress} 
					max="100"
					className="w-full h-1 rounded-full bg-gray-800 [&::-webkit-progress-bar]:bg-gray-800 [&::-webkit-progress-bar]:rounded-full [&::-webkit-progress-value]:bg-red-500 [&::-webkit-progress-value]:rounded-full [&::-moz-progress-bar]:bg-red-500 [&::-moz-progress-bar]:rounded-full"
				/>
			</div>
		)}
	</div>
)