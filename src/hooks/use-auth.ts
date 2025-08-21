import { useState, useEffect, useRef } from "react"
import { authService } from "../services/auth-service"
import { discordService } from "../services/discord-service"
import { navigationUtils } from "../utils/navigation-utils"
import { SignInFormData, AuthState } from "../types/auth"

export const useAuth = () => {
    // Form state
    const [formData, setFormData] = useState<SignInFormData>({
        email: "",
        password: "",
        rememberMe: false
    })
    
    const [showPassword, setShowPassword] = useState(false)
    
    // Auth state
    const [authState, setAuthState] = useState<AuthState>({
        isLoading: true, // Start with true to check for saved login
        isTransitioning: false,
        error: null
    })

    // Prevent double execution
    const hasCheckedSavedLogin = useRef(false)

    // Check for saved login on mount
    useEffect(() => {
        const checkSavedLogin = async () => {
            if (hasCheckedSavedLogin.current) return
            hasCheckedSavedLogin.current = true

            try {
                await discordService.initializeDiscordRPC()
                await discordService.setIdle()
            } catch (error) {
                // Error initializing Discord RPC, log it but continue
                console.log("Error initializing Discord RPC:", error)
            }

            try {
                await authService.fetchSavedUser()
                
                // User has saved login, transition to launcher
                setAuthState(prev => ({ 
                    ...prev, 
                    isLoading: false,
                    isTransitioning: true 
                }))
                
                // Add delay to ensure transition animation shows
                setTimeout(async () => {
                    await navigationUtils.navigateToLauncher()
                }, 300)
                
            } catch (error) {
                // No saved login found, show sign-in form
                setAuthState(prev => ({ 
                    ...prev, 
                    isLoading: false 
                }))
            }
        }

        // Small delay to prevent flash
        const timer = setTimeout(checkSavedLogin, 100)
        return () => clearTimeout(timer)
    }, [])

    // Form handlers
    const updateFormData = (field: keyof SignInFormData, value: string | boolean) => {
        setFormData(prev => ({
            ...prev,
            [field]: value
        }))
    }

    const toggleShowPassword = () => {
        setShowPassword(prev => !prev)
    }

    // Auth actions
    const handleSignIn = async (e: React.FormEvent) => {
        e.preventDefault()
        
        setAuthState(prev => ({ 
            ...prev, 
            isLoading: true, 
            error: null 
        }))

        try {
            await authService.signIn(formData)

            setAuthState(prev => ({ 
                ...prev, 
                isLoading: false,
                isTransitioning: true 
            }))

            // Add delay to ensure transition animation shows
            setTimeout(async () => {
                await navigationUtils.navigateToLauncher()
            }, 300)

        } catch (error) {
            setAuthState(prev => ({ 
                ...prev, 
                isLoading: false,
                error: String(error) 
            }))
        }
    }

    const handleForgotPassword = () => {
        navigationUtils.openExternal("https://realityfn.org/forgot-password")
    }

    const handleCreateAccount = () => {
        navigationUtils.openExternal("https://realityfn.org/sign-up")
    }

    return {
        // State
        formData,
        showPassword,
        authState,
        
        // Actions
        updateFormData,
        toggleShowPassword,
        handleSignIn,
        handleForgotPassword,
        handleCreateAccount
    }
}