import { invoke } from "@tauri-apps/api/core"
import { AccountInfo, SignInFormData } from "../types/auth"

export const authService = {
    /**
     * Fetch saved user login information
     */
    async fetchSavedUser(): Promise<AccountInfo> {
        try {
            console.log("Fetching saved login data...")
            const account_info = await invoke<AccountInfo>("fetchsu")
            console.log("Successfully fetched saved login data", account_info)
            return account_info
        } catch (error) {
            console.log("No saved login data found", error)
            throw error
        }
    },

    /**
     * Sign in user with email and password
     */
    async signIn({ email, password, rememberMe }: SignInFormData): Promise<AccountInfo> {
        try {
            console.log("Attempting sign in for:", email)
            
            const account_info = await invoke<AccountInfo>("loginu", {
                email,
                password
            })

            console.log("Sign in successful")

            if (rememberMe) {
                await this.saveUserCredentials(account_info.RefreshToken)
            }

            return account_info
        } catch (error) {
            console.error("Sign in failed:", error)
            throw error
        }
    },

    /**
     * Save user credentials for remember me functionality
     */
    async saveUserCredentials(refreshToken: string): Promise<void> {
        try {
            await invoke("saveu", {
                enableRememberMe: true,
                refreshToken
            })
            console.log("User credentials saved")
        } catch (error) {
            console.error("Failed to save user credentials:", error)
            throw error
        }
    }
}