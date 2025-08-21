import { openUrl } from '@tauri-apps/plugin-opener'

/**
 * Navigation utilities that work reliably in both development and production
 */

export const navigationUtils = {
    /**
     * Navigate to launcher page
     */
    async navigateToLauncher(): Promise<void> {
        if (typeof window !== 'undefined') {
            // Dispatch a custom event that the main app listens for
            const navigateEvent = new CustomEvent('navigate-to-launcher', {
                detail: { target: 'launcher' }
            })
            window.dispatchEvent(navigateEvent)
            
            // Set a flag in sessionStorage as a backup
            try {
                sessionStorage.setItem('showLauncher', 'true')
            } catch (error) {
                // sessionStorage might not be available in some contexts
            }
            
            // Try to reload the page to ensure the app updates
            // This is a fallback in case the event listener doesn't work
            setTimeout(() => {
                if (window.location.pathname !== '/') {
                    window.location.href = '/'
                }
            }, 500)
        }
    },

    /**
     * Fallback navigation method
     */
    fallbackNavigation(): void {
        if (typeof window !== 'undefined') {
            setTimeout(() => {
                window.location.href = '/'
            }, 100)
        }
    },

    /**
     * Open external URL using Tauri's opener plugin
     */
    async openExternal(url: string): Promise<void> {
        try {
            await openUrl(url)
        } catch (error) {
            console.warn("Failed to open with Tauri opener, using window.open:", error)
            if (typeof window !== 'undefined') {
                window.open(url, "_blank")
            }
        }
    }
}