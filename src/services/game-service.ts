import { invoke } from "@tauri-apps/api/core"
import { InstalledObject, ProgressUpdate, InstallLocation } from "../types/installation"
import { Game } from "../types/game"

export class GameService {
	static async checkGameInstallation(artifactId: string): Promise<{ installed: boolean; location?: string }> {
		try {
			const installedObject: InstalledObject = await invoke<InstalledObject>(
				"fetch_installed_object_by_artifact_id",
				{ artifactId }
			)
			return {
				installed: true,
				location: installedObject.installation_location
			}
		} catch (error) {
			return { installed: false }
		}
	}

	static async getInstallLocations(): Promise<InstallLocation[]> {
		return await invoke<InstallLocation[]>("get_drives")
	}

	static async getProgressUpdates(): Promise<ProgressUpdate[]> {
		return await invoke<ProgressUpdate[]>("get_progress")
	}

	static async startDownload(installDir: string): Promise<void> {
		await invoke("start_download", { installDir })
	}

	static async startVerify(): Promise<void> {
		await invoke("start_verify")
	}

	static async startUninstall(): Promise<void> {
		await invoke("start_uninstall")
	}

	static async cancelDownload(installDir: string): Promise<void> {
		console.log("Cancelling download for:", installDir)
		await invoke("cancel_download", { installDir })
	}

	static async downloadComplete(installDir: string): Promise<void> {
		await invoke("download_complete", { installDir })
	}

	static async uninstallComplete(): Promise<void> {
		await invoke("uninstall_complete")
	}

	static async launchGame(): Promise<void> {
		await invoke("launch_game")
	}

	static async fetchGameData(): Promise<Game> {
		try {
			return await invoke<Game>("get_game_information")
		}
		catch (error) {
			console.error("Error fetching game data:", error)
			throw error
		}
	}
}