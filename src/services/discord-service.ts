import { invoke } from "@tauri-apps/api/core"

export const discordService = {
    /**
     * Initialize Discord RPC connection
     */
    async initializeDiscordRPC(): Promise<void> {
        try {
            console.log("Initializing Discord RPC...")
            await invoke("initialize_discord_rpc")
            console.log("Initialized Discord RPC")
        } catch (error) {
            console.log("Error initializing Discord RPC", error)
            throw error
        }
    },

    /**
     * Disconnect Discord RPC connection
     */
    async disconnectDiscordRPC(): Promise<void> {
        try {
            console.log("Disconnecting Discord RPC...")
            await invoke("disconnect_discord_rpc")
            console.log("Disconnected Discord RPC")
        } catch (error) {
            console.log("Error disconnecting Discord RPC", error)
            throw error
        }
    },

    /**
     * Set Discord RPC activity to idle
     */
    async setIdle(): Promise<void> {
        try {
            console.log("Setting Discord RPC to idle...")
            await invoke("set_idle_activity")
            console.log("Set Discord RPC to idle")
        } catch (error) {
            console.log("Error setting Discord RPC to idle", error)
            throw error
        }
    },

    /**
     * Set Discord RPC activity to playing
     */
    async setPlaying(): Promise<void> {
        try {
            console.log("Setting Discord RPC to playing...")
            await invoke("set_playing_activity")
            console.log("Set Discord RPC to playing")
        } catch (error) {
            console.log("Error setting Discord RPC to playing", error)
            throw error
        }
    }
}