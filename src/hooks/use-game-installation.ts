import { useState, useCallback, useRef, useEffect } from "react"
import { GameService } from "../services/game-service"
import { InstallState, VerifyState, UninstallState } from "../types/launcher"

export const useGameInstallation = (
	gameId: string,
	onError: (title: string, message: string) => void
) => {
	const [gameInstalled, setGameInstalled] = useState(false)
	const [installLocation, setInstallLocation] = useState("")
	const [installState, setInstallState] = useState<InstallState>({
		isInstalling: false,
		progress: 0,
		location: ""
	})
	const [verifyState, setVerifyState] = useState<VerifyState>({
		isVerifying: false,
		progress: 0,
		result: null
	})
	const [uninstallState, setUninstallState] = useState<UninstallState>({
		isUninstalling: false,
		progress: 0,
		result: null
	})

	const installIntervalRef = useRef<number | null>(null)
	const verifyIntervalRef = useRef<number | null>(null)
	const uninstallIntervalRef = useRef<number | null>(null)

	const checkInstallation = useCallback(async () => {
		try {
			const result = await GameService.checkGameInstallation(gameId)
			setGameInstalled(result.installed)
			if (result.location) {
				setInstallLocation(result.location)
			}
		} catch (error) {
			console.log("Error checking installation:", error)
		}
	}, [])

	const monitorInstallProgress = useCallback(async () => {
		try {
			const updates = await GameService.getProgressUpdates()
			if (!updates.length) return

			const last = updates[updates.length - 1]
			
			// Check if this is a completion signal (exact match or completion message)
			const isComplete = last.downloaded_bytes >= last.total_bytes || 
				last.filename.toLowerCase().includes('complete') ||
				last.filename.toLowerCase().includes('done')
			
			const percent = isComplete ? 100 : Math.min((last.downloaded_bytes / last.total_bytes) * 100, 100)

			setInstallState(prev => ({ ...prev, progress: percent }))

			if (percent >= 100 || isComplete) {
				if (installIntervalRef.current) {
					clearInterval(installIntervalRef.current)
					installIntervalRef.current = null
				}

				setInstallState(prev => ({ ...prev, isInstalling: false, progress: 100 }))
				setGameInstalled(true)

				try {
					await GameService.downloadComplete(installLocation)
				} catch (error) {
					onError("Installation Complete Error", "Installation finished but failed to finalize.")
				}
			}
		} catch (error) {
			console.error("Failed to fetch progress:", error)
		}
	}, [installLocation, onError])

	const monitorVerifyProgress = useCallback(async () => {
		try {
			const updates = await GameService.getProgressUpdates()
			if (!updates.length) return

			// Look at all recent updates, not just the last one
			const recentUpdates = updates.slice(-5) // Last 5 updates
			const last = updates[updates.length - 1]
			
			// Check for completion signals in any of the recent updates
			const hasCompletionSignal = recentUpdates.some(update => 
				update.downloaded_bytes >= update.total_bytes ||
				update.filename.toLowerCase().includes('complete') ||
				update.filename.toLowerCase().includes('done') ||
				update.filename.toLowerCase().includes('verification complete')
			)
			
			// Calculate progress
			const rawPercent = (last.downloaded_bytes / last.total_bytes) * 100
			const percent = hasCompletionSignal ? 100 : Math.min(rawPercent, 100)

			console.log(`Verify progress: ${last.downloaded_bytes}/${last.total_bytes} = ${rawPercent.toFixed(2)}% -> ${percent}% (completion signal: ${hasCompletionSignal}, filename: "${last.filename}")`)

			setVerifyState(prev => ({ ...prev, progress: percent }))

			// Complete if we hit 100% OR if we detect completion signal OR if we're very close (99.5%+)
			if (percent >= 100 || hasCompletionSignal || rawPercent >= 99.5) {
				console.log("Verification completing...")
				
				if (verifyIntervalRef.current) {
					clearInterval(verifyIntervalRef.current)
					verifyIntervalRef.current = null
				}

				setVerifyState({
					isVerifying: false,
					progress: 100,
					result: "success"
				})

				setTimeout(() => {
					setVerifyState(prev => ({ ...prev, result: null }))
				}, 3000)
			}
		} catch (error) {
			console.error("Verification progress error:", error)
			if (verifyIntervalRef.current) {
				clearInterval(verifyIntervalRef.current)
				verifyIntervalRef.current = null
			}
			setVerifyState({
				isVerifying: false,
				progress: 0,
				result: "error"
			})
			onError("Verification Error", "Failed to verify game files.")
		}
	}, [onError])

	const monitorUninstallProgress = useCallback(async () => {
		try {
			const updates = await GameService.getProgressUpdates()
			if (!updates.length) return

			// Look at all recent updates, not just the last one (like verify does)
			const recentUpdates = updates.slice(-5) // Last 5 updates
			const last = updates[updates.length - 1]
			
			// Check for completion signals in any of the recent updates
			const hasCompletionSignal = recentUpdates.some(update => 
				update.downloaded_bytes >= update.total_bytes ||
				update.filename.toLowerCase().includes('complete') ||
				update.filename.toLowerCase().includes('done') ||
				update.filename.toLowerCase().includes('uninstall complete')
			)
			
			// Calculate progress
			const rawPercent = (last.downloaded_bytes / last.total_bytes) * 100
			const percent = hasCompletionSignal ? 100 : Math.min(rawPercent, 100)

			console.log(`Uninstall progress: ${last.downloaded_bytes}/${last.total_bytes} = ${rawPercent.toFixed(2)}% -> ${percent}% (completion signal: ${hasCompletionSignal}, filename: "${last.filename}")`)

			setUninstallState(prev => ({ ...prev, progress: percent }))

			// Complete if we hit 100% OR if we detect completion signal OR if we're very close (99.5%+)
			if (percent >= 100 || hasCompletionSignal || rawPercent >= 99.5) {
				console.log("Uninstallation completing...")
				
				if (uninstallIntervalRef.current) {
					clearInterval(uninstallIntervalRef.current)
					uninstallIntervalRef.current = null
				}

				setUninstallState({
					isUninstalling: false,
					progress: 100,
					result: "success"
				})

				setTimeout(async() => {
					setUninstallState(prev => ({ ...prev, result: null }))
					setGameInstalled(false)

					try {
						await GameService.uninstallComplete()
					} catch (error) {
						onError("Uninstallation Complete Error", "Uninstallation finished but failed to finalize.")
					}
				}, 3000)
			}
		} catch (error) {
			console.error("Uninstallation progress error:", error)
			if (uninstallIntervalRef.current) {
				clearInterval(uninstallIntervalRef.current)
				uninstallIntervalRef.current = null
			}
			setUninstallState({
				isUninstalling: false,
				progress: 0,
				result: "error"
			})
			onError("Uninstallation Error", "Failed to uninstall game files.")
		}
	}, [onError])

	const startInstallation = useCallback(async (path: string) => {
		setInstallState({
			isInstalling: true,
			progress: 0,
			location: path
		})
		setInstallLocation(path)

		try {
			installIntervalRef.current = window.setInterval(monitorInstallProgress, 300)
			await GameService.startDownload(path)
		} catch (error) {
			if (installIntervalRef.current) {
				clearInterval(installIntervalRef.current)
				installIntervalRef.current = null
			}
			setInstallState({ isInstalling: false, progress: 0, location: "" })
			onError("Installation Error", `Failed to start installation: ${error}`)
		}
	}, [monitorInstallProgress, onError])

	const cancelInstallation = useCallback(async () => {
		if (installIntervalRef.current) {
			clearInterval(installIntervalRef.current)
			installIntervalRef.current = null
		}

		try {
			await GameService.cancelDownload(installState.location);
		} catch (error) {
			console.error("Error cancelling download:", error)
			onError("Cancellation Error", `Failed to cancel installation: ${error}`)
		}

		setInstallState({ isInstalling: false, progress: 0, location: "" })
	}, [installState.location, onError])

	const startVerification = useCallback(async () => {
		if (!gameInstalled) return

		setVerifyState({
			isVerifying: true,
			progress: 0,
			result: null
		})

		try {
			verifyIntervalRef.current = window.setInterval(monitorVerifyProgress, 300)
			await GameService.startVerify()
		} catch (error) {
			if (verifyIntervalRef.current) {
				clearInterval(verifyIntervalRef.current)
				verifyIntervalRef.current = null
			}
			setVerifyState({
				isVerifying: false,
				progress: 0,
				result: "error"
			})
			onError("Verification Error", `Failed to start verification: ${error}`)
		}
	}, [gameInstalled, monitorVerifyProgress, onError])

	const startUninstallation = useCallback(async () => {
		if (!gameInstalled) return

		setUninstallState({
			isUninstalling: true,
			progress: 0,
			result: null
		})

		try {
			uninstallIntervalRef.current = window.setInterval(monitorUninstallProgress, 300)
			await GameService.startUninstall()
		} catch (error) {
			if (uninstallIntervalRef.current) {
				clearInterval(uninstallIntervalRef.current)
				uninstallIntervalRef.current = null
			}
			setUninstallState({
				isUninstalling: false,
				progress: 0,
				result: "error"
			})
			onError("Uninstallation Error", `Failed to start uninstallation: ${error}`)
		}
	}, [gameInstalled, monitorUninstallProgress, onError])

	useEffect(() => {
		return () => {
			if (installIntervalRef.current) {
				clearInterval(installIntervalRef.current)
			}
			if (verifyIntervalRef.current) {
				clearInterval(verifyIntervalRef.current)
			}
			if (uninstallIntervalRef.current) {
				clearInterval(uninstallIntervalRef.current)
			}
		}
	}, [])

	return {
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
	}
}