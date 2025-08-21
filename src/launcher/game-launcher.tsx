"use client"

import { useState, useEffect } from "react"
import { Button } from "../components/ui/button"
import { fortniteApi } from "../services/fortnite-api"
import { Avatar, AvatarFallback, AvatarImage } from "../components/ui/avatar"
import { Badge } from "../components/ui/badge"
import {
	Library,
	HeartIcon as Friends
} from "lucide-react"

import { type AccountInfo } from "../types/auth"
import { type InstallLocation } from "../types/installation"
import { Game } from "../types/game"
import { invoke } from "@tauri-apps/api/core"

// Import hooks
import { useErrorModal } from "../hooks/use-error-modal"
import { useGameInstallation } from "../hooks/use-game-installation"
import { useGameLauncher } from "../hooks/use-game-launcher"

// Import components
import { ErrorModal } from "../components/error-modal"
import { InstallModal } from "../components/install-modal"
import { GameActions } from "../components/game-actions"
import { GameInfo } from "../components/game-info"
import { QuickActions } from "../components/quick-actions"

// Import services
import { GameService } from "../services/game-service"

// Import pages
import FriendsPage from "./pages/friends"
import SettingsPage from "./pages/settings"

function Launcher() {
	const [selectedCategory, setSelectedCategory] = useState("library")
	const [isLoaded, setIsLoaded] = useState(false)
	const [accountInfo, setAccountInfo] = useState<AccountInfo | null>(null)
	const [game, setGame] = useState<Game | null>(null)
	const [showInstallModal, setShowInstallModal] = useState(false)
	const [defaultInstallLocations, setDefaultInstallLocations] = useState<InstallLocation[]>([])

	// Custom hooks
	const { errorState, showError, closeError } = useErrorModal()
	const { isPlaying, launchGame } = useGameLauncher(showError)
	const {
		gameInstalled,
		installLocation,
		installState,
		verifyState,
    uninstallState,
		checkInstallation,
		startInstallation,
		cancelInstallation,
		startVerification,
    startUninstallation
	} = useGameInstallation("REALITY_ALPHA_TEST_ID", showError)

	// Initialize launcher data
	useEffect(() => {
		const timer = setTimeout(async () => {
			try {
				const [accountData, drives, gameData] = await Promise.all([
					invoke<AccountInfo>("fetchcu"),
					GameService.getInstallLocations(),
					GameService.fetchGameData()
				])

				setAccountInfo(accountData)
				setDefaultInstallLocations(drives)
				setGame(gameData)
				await checkInstallation()
			} catch (error) {
				showError("Initialization Error", "Failed to load launcher data.")
			} finally {
				setIsLoaded(true)
			}
		}, 100)

		return () => clearTimeout(timer)
	}, [checkInstallation, showError])

	const handleInstallGame = () => {
		setShowInstallModal(true)
	}

	const handleInstallLocationConfirm = (path: string) => {
		setShowInstallModal(false)
		startInstallation(path)
	}

	const handleLaunchGame = () => {
		if (game) {
			launchGame(game.title)
		}
	}

	const renderMainContent = () => {
		switch (selectedCategory) {
			case "friends":
				return <FriendsPage />
			case "settings":
				return <SettingsPage />
			default:
				return renderLibraryContent()
		}
	}

	const renderLibraryContent = () => {
		if (!game) return <div>Loading...</div>

		return (
			<div className="flex flex-col h-full">
				<div
					className={`backdrop-blur-xl bg-black/20 border-b border-gray-800/30 p-4 flex-shrink-0 transition-all duration-700 delay-300 ${
						isLoaded ? "opacity-100 translate-y-0" : "opacity-0 -translate-y-4"
					}`}
				>
					<div className="flex items-center justify-between">
						<div>
							<h1 className="text-2xl font-bold text-gray-100">Game Library</h1>
							<p className="text-gray-400 text-sm">Featured</p>
						</div>
					</div>
				</div>

				<div className="flex-1 overflow-y-auto p-6">
					<div className="max-w-6xl mx-auto">
						<div
							className={`backdrop-blur-xl bg-black/30 rounded-2xl border border-gray-800/50 overflow-hidden mb-6 transition-all duration-800 delay-500 ${
								isLoaded ? "opacity-100 scale-100 translate-y-0" : "opacity-0 scale-95 translate-y-8"
							}`}
						>
							<div className="relative">
								<img src={game.image || "/placeholder.svg"} alt={game.title} className="w-full h-80 object-cover" />
								<div className="absolute inset-0 bg-gradient-to-t from-black via-black/20 to-transparent" />

								<div className="absolute bottom-0 left-0 right-0 p-8">
									<div className="flex items-end justify-between">
										<div>
											<Badge className="bg-cyan-500/20 text-cyan-300 border-cyan-500/30 mb-3">{game.badge}</Badge>
											<h2 className="text-4xl font-bold text-white mb-2">{game.title}</h2>
										</div>

										<GameActions
											gameInstalled={gameInstalled}
											isPlaying={isPlaying}
											installState={installState}
											onLaunchGame={handleLaunchGame}
											onInstallGame={handleInstallGame}
											onCancelInstall={cancelInstallation}
										/>
									</div>
								</div>
							</div>
						</div>

						<div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
							<div className="lg:col-span-2 space-y-6">
								<div
									className={`backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-6 transition-all duration-700 delay-700 ${
										isLoaded ? "opacity-100 translate-y-0" : "opacity-0 translate-y-6"
									}`}
								>
									<h3 className="text-xl font-semibold text-gray-100 mb-4">About</h3>
									<p className="text-gray-300 leading-relaxed">{game.description}</p>
								</div>

								<div
									className={`backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-6 transition-all duration-700 delay-900 ${
										isLoaded ? "opacity-100 translate-y-0" : "opacity-0 translate-y-6"
									}`}
								>
									<h3 className="text-xl font-semibold text-gray-100 mb-4">Screenshots</h3>
									<div className="grid grid-cols-3 gap-4">
										{game.screenshots.map((screenshot, index) => (
											<img
												key={index}
												src={screenshot || "/placeholder.svg"}
												alt={`Screenshot ${index + 1}`}
												className={`w-full h-24 object-cover rounded-lg border border-gray-800/50 hover:scale-105 transition-all cursor-pointer duration-500 ${
													isLoaded ? "opacity-100 scale-100" : "opacity-0 scale-90"
												}`}
												style={{ transitionDelay: `${1100 + index * 100}ms` }}
											/>
										))}
									</div>
								</div>
							</div>

							<div className="space-y-6">
								<div
									className={`backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-6 transition-all duration-700 delay-700 ${
										isLoaded ? "opacity-100 translate-x-0" : "opacity-0 translate-x-6"
									}`}
								>
									<h3 className="text-lg font-semibold text-gray-100 mb-4">Game Info</h3>
									<GameInfo 
										game={game}
										gameInstalled={gameInstalled}
										isInstalling={installState.isInstalling}
										installLocation={installLocation}
									/>
								</div>

								<div
									className={`backdrop-blur-xl bg-black/30 rounded-xl border border-gray-800/50 p-6 transition-all duration-700 delay-900 ${
										isLoaded ? "opacity-100 translate-x-0" : "opacity-0 translate-x-6"
									}`}
								>
									<h3 className="text-lg font-semibold text-gray-100 mb-4">Quick Actions</h3>
									<QuickActions
										gameInstalled={gameInstalled}
										verifyState={verifyState}
                    uninstallState={uninstallState}
										onVerifyFiles={startVerification}
                    onUninstallGame={startUninstallation}
									/>
								</div>
							</div>
						</div>
					</div>
				</div>

				<InstallModal
					isOpen={showInstallModal}
					onClose={() => setShowInstallModal(false)}
					game={game}
					defaultLocations={defaultInstallLocations}
					onInstall={handleInstallLocationConfirm}
				/>

				<ErrorModal
					isOpen={errorState.isOpen}
					onClose={closeError}
					title={errorState.title}
					message={errorState.message}
				/>
			</div>
		)
	}

	return (
		<div
			className={`min-h-screen bg-gradient-to-br from-black via-gray-900 to-black relative transition-all duration-1000 ${
				isLoaded ? "opacity-100" : "opacity-0"
			}`}
		>
			<div className="absolute inset-0 bg-[url('/placeholder.svg?height=1080&width=1920')] bg-cover bg-center opacity-5" />
			<div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />

			<div className="relative z-10 flex h-screen">
				<div
					className={`w-64 backdrop-blur-xl bg-black/40 border-r border-gray-800/50 p-4 flex flex-col transition-all duration-800 delay-200 ${
						isLoaded ? "opacity-100 -translate-x-0" : "opacity-0 -translate-x-8"
					}`}
				>
					<div
						className={`flex items-center gap-3 p-3 rounded-xl backdrop-blur-sm bg-black/30 border border-gray-800/50 mb-6 transition-all duration-600 delay-400 ${
							isLoaded ? "opacity-100 scale-100" : "opacity-0 scale-95"
						}`}
					>
						<Avatar className="h-10 w-10">
							<AvatarImage src="/placeholder.svg?height=40&width=40" />
							<AvatarFallback className="bg-gray-800 text-gray-300">
								{accountInfo
									? accountInfo.DisplayName.split(" ")
											.filter((x) => !(x.includes("[") && x.includes("]")))
											.map((n) => n[0])
											.join("")
									: "U"}
							</AvatarFallback>
						</Avatar>
						<div className="flex-1 min-w-0">
							<p className="text-gray-100 font-medium text-sm truncate">
								{isLoaded && accountInfo ? accountInfo.DisplayName : "Unknown"}
							</p>
							<p className="text-gray-400 text-xs">Level {fortniteApi.getGameLevel(accountInfo)}</p>
						</div>
					</div>

					<nav className="space-y-2 flex-1">
						{[
							{ id: "library", label: "Library", icon: Library },
							{ id: "friends", label: "Friends", icon: Friends }
						].map((item, index) => (
							<Button
								key={item.id}
								variant="ghost"
								className={`w-full justify-start gap-3 text-gray-300 hover:text-gray-100 hover:bg-black/20 transition-all duration-500 ${
									selectedCategory === item.id ? "bg-black/40 text-gray-100 border border-gray-800/50" : ""
								} ${isLoaded ? "opacity-100 translate-x-0" : "opacity-0 -translate-x-4"}`}
								style={{ transitionDelay: `${600 + index * 100}ms` }}
								onClick={() => setSelectedCategory(item.id)}
							>
								<item.icon className="h-4 w-4" />
								{item.label}
							</Button>
						))}
					</nav>
				</div>

				<div
					className={`flex-1 transition-all duration-900 delay-400 overflow-hidden ${
						isLoaded ? "opacity-100 translate-x-0" : "opacity-0 translate-x-8"
					}`}
				>
					{renderMainContent()}
				</div>
			</div>
		</div>
	)
}

export default Launcher