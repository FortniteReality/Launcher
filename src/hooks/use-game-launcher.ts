import { useState, useCallback, useEffect, useRef } from "react"
import { invoke } from "@tauri-apps/api/core"
import { getCurrentWindow } from "@tauri-apps/api/window"
import { GameService } from "../services/game-service"

export const useGameLauncher = (
	onError: (title: string, message: string) => void
) => {
	const [isPlaying, setIsPlaying] = useState(false)
	const [isMinimized, setIsMinimized] = useState(false)
	const pollingIntervalRef = useRef<number | null>(null)
	const isPlayingRef = useRef(false)
	const isMinimizedRef = useRef(false)

	useEffect(() => {
		isPlayingRef.current = isPlaying
	}, [isPlaying])

	useEffect(() => {
		isMinimizedRef.current = isMinimized
	}, [isMinimized])
	const checkGameStatus = useCallback(async () => {
		try {
			const result = await invoke<boolean>("get_is_playing")
			return result
		} catch (error) {
			console.error("Failed to check game status:", error)
			return false
		}
	}, [])

	const minimizeWindow = useCallback(async () => {
		try {
			const window = getCurrentWindow()
			await window.minimize()
			setIsMinimized(true)
		} catch (error) {
			console.error("Failed to minimize window:", error)
			onError("Window Error", "Failed to minimize window")
		}
	}, [onError])

	const restoreWindow = useCallback(async () => {
		try {
			const window = getCurrentWindow()
			await window.unminimize()
			await window.setFocus()
			setIsMinimized(false)
		} catch (error) {
			console.error("Failed to restore window:", error)
		}
	}, [])

	const startPolling = useCallback(() => {
		if (pollingIntervalRef.current) {
			clearInterval(pollingIntervalRef.current)
		}

		pollingIntervalRef.current = setInterval(async () => {
			const gameIsPlaying = await checkGameStatus()
			
			if (!gameIsPlaying && isPlayingRef.current) {
				setIsPlaying(false)
				await invoke("set_idle_activity")
				
				if (isMinimizedRef.current) {
					await restoreWindow()
				}
				
				if (pollingIntervalRef.current) {
					clearInterval(pollingIntervalRef.current)
					pollingIntervalRef.current = null
				}
			}
		}, 3000)
	}, [checkGameStatus, restoreWindow])

	const stopPolling = useCallback(() => {
		if (pollingIntervalRef.current) {
			clearInterval(pollingIntervalRef.current)
			pollingIntervalRef.current = null
		}
	}, [])

	const launchGame = useCallback(async (gameTitle: string) => {
		try {
			console.log(`Launching ${gameTitle}...`)
			setIsPlaying(true)
			GameService.launchGame()

			await new Promise(resolve => setTimeout(resolve, 8000))

			await invoke("set_playing_activity")
			
			startPolling()
			await minimizeWindow()
		} catch (error) {
			setIsPlaying(false)
			stopPolling()
			onError("Launch Error", `Failed to launch ${gameTitle}: ${error}`)
		}
	}, [onError, startPolling, stopPolling, minimizeWindow])

	useEffect(() => {
		return () => {
			stopPolling()
		}
	}, [stopPolling])

	const minimize = useCallback(async () => {
		if (isPlaying) {
			await minimizeWindow()
		}
	}, [isPlaying, minimizeWindow])

	return {
		isPlaying,
		isMinimized,
		launchGame,
		minimize,
		stopPolling
	}
}