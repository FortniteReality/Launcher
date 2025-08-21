"use client"

import { useAuth } from "../../hooks/use-auth"
import LoadingScreen from "../../components/loading-screen"
import SignInForm from "../../components/sign-in-form"

export default function SignInPage() {
    const {
        formData,
        showPassword,
        authState,
        updateFormData,
        toggleShowPassword,
        handleSignIn,
        handleForgotPassword,
        handleCreateAccount
    } = useAuth()

    // Show loading screen while checking for saved login
    if (authState.isLoading) {
        return (
            <div className="min-h-screen bg-gradient-to-br from-black via-gray-900 to-black flex items-center justify-center">
                <div className="text-center">
                    <div className="w-16 h-16 border-4 border-blue-500/30 border-t-blue-500 rounded-full animate-spin mx-auto mb-4"></div>
                    <h2 className="text-2xl font-bold text-gray-100 mb-2">Checking Login</h2>
                    <p className="text-gray-400">Please wait...</p>
                </div>
            </div>
        )
    }

    return (
        <div className="min-h-screen bg-gradient-to-br from-black via-gray-900 to-black relative overflow-hidden">
            <div className="absolute inset-0 bg-[url('/placeholder.svg?height=1080&width=1920')] bg-cover bg-center opacity-5" />
            <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />

            <LoadingScreen isVisible={authState.isTransitioning} />

            <div className="relative z-10 flex items-center justify-center min-h-screen p-6">
                <SignInForm
                    formData={formData}
                    showPassword={showPassword}
                    isLoading={authState.isLoading}
                    error={authState.error}
                    onFormDataChange={updateFormData}
                    onShowPasswordToggle={toggleShowPassword}
                    onSubmit={handleSignIn}
                    onForgotPassword={handleForgotPassword}
                    onCreateAccount={handleCreateAccount}
                />
            </div>
        </div>
    )
}