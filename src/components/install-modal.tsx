import React, { useState, useEffect } from "react"
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from "./ui/dialog"
import { Button } from "./ui/button"
import { Folder, HardDrive } from "lucide-react"
import { open } from '@tauri-apps/plugin-dialog'
import { Game } from "../types/game"
import { InstallLocation } from "../types/installation"

interface InstallModalProps {
	isOpen: boolean
	onClose: () => void
	game: Game
	defaultLocations: InstallLocation[]
	onInstall: (path: string) => void
}

export const InstallModal: React.FC<InstallModalProps> = ({
	isOpen,
	onClose,
	game,
	defaultLocations,
	onInstall
}) => {
	const [selectedLocation, setSelectedLocation] = useState("")
	const [customPath, setCustomPath] = useState("")

	useEffect(() => {
		if (defaultLocations.length > 0 && !selectedLocation) {
			setSelectedLocation(defaultLocations[0].path)
		}
	}, [defaultLocations, selectedLocation])

	const handleInstall = () => {
		const finalPath = customPath || selectedLocation
		onInstall(finalPath)
	}

	const handleBrowseCustom = async () => {
		try {
			const selectedPath = await open({
				directory: true,
				multiple: false,
				title: "Select Installation Folder",
			})

			if (selectedPath) {
				const finalPath = `${selectedPath}\\${game.title}`
				setCustomPath(finalPath)
				onInstall(finalPath)
			}
		} catch (error) {
			console.error("Error opening folder dialog:", error)
		}
	}

	return (
		<Dialog open={isOpen} onOpenChange={onClose}>
			<DialogContent className="backdrop-blur-xl bg-black/90 border-gray-800/50 text-gray-100">
				<DialogHeader>
					<DialogTitle className="text-gray-100">Choose Install Location</DialogTitle>
					<DialogDescription className="text-gray-400">
						Select where you want to install {game.title}
					</DialogDescription>
				</DialogHeader>

				<div className="space-y-4">
					{defaultLocations.map((location) => (
						<div
							key={location.path}
							className={`p-4 rounded-lg border cursor-pointer transition-all ${
								selectedLocation === location.path
									? "border-blue-500 bg-blue-500/10"
									: "border-gray-700 bg-black/20 hover:bg-black/30"
							}`}
							onClick={() => setSelectedLocation(location.path)}
						>
							<div className="flex items-center justify-between">
								<div className="flex items-center gap-3">
									<HardDrive className="h-5 w-5 text-gray-400" />
									<div>
										<p className="font-medium text-gray-100">{location.label}</p>
										<p className="text-sm text-gray-400">{location.path}</p>
									</div>
								</div>
								<div className="text-right">
									<p className="text-sm text-gray-300">{location.free} free</p>
									<p className="text-xs text-gray-500">of {location.total}</p>
								</div>
							</div>
						</div>
					))}

					<div className="border-t border-gray-700 pt-4">
						<p className="text-sm text-gray-400 mb-2">Custom Location:</p>
						<Button
							variant="outline"
							className="w-full border-gray-700 text-gray-300 hover:bg-black/20 bg-transparent justify-start"
							onClick={handleBrowseCustom}
						>
							<Folder className="h-4 w-4 mr-2" />
							{customPath || "Browse for folder..."}
						</Button>
					</div>
				</div>

				<DialogFooter>
					<Button
						variant="outline"
						onClick={onClose}
						className="border-gray-700 text-gray-300 hover:bg-black/20"
					>
						Cancel
					</Button>
					<Button onClick={handleInstall} className="bg-blue-600 hover:bg-blue-700">
						Install Here
					</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	)
}