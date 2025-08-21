import type React from "react"
import { Button } from "./ui/button"
import { Input } from "./ui/input"
import { Eye, EyeOff, User, Lock, Mail, AlertCircle } from "lucide-react"
import { SignInFormData } from "../types/auth"

interface SignInFormProps {
    formData: SignInFormData
    showPassword: boolean
    isLoading: boolean
    error: string | null
    onFormDataChange: (field: keyof SignInFormData, value: string | boolean) => void
    onShowPasswordToggle: () => void
    onSubmit: (e: React.FormEvent) => void
    onForgotPassword: () => void
    onCreateAccount: () => void
}

export default function SignInForm({
    formData,
    showPassword,
    isLoading,
    error,
    onFormDataChange,
    onShowPasswordToggle,
    onSubmit,
    onForgotPassword,
    onCreateAccount
}: SignInFormProps) {
    return (
        <div className="w-full max-w-md">
            <div className="backdrop-blur-xl bg-black/40 border border-gray-800/50 rounded-2xl p-8 shadow-2xl">
                <div className="text-center mb-8">
                    <div className="flex items-center justify-center mb-4">
                        <div className="backdrop-blur-sm bg-black/30 border border-gray-800/50 rounded-full p-4">
                            <User className="h-8 w-8 text-gray-300" />
                        </div>
                    </div>
                    <h1 className="text-3xl font-bold text-gray-100 mb-2">Welcome Back</h1>
                    <p className="text-gray-400">Sign in to access Reality</p>
                </div>

                <form onSubmit={onSubmit} className="space-y-6">
                    {error && (
                        <div className="flex items-center gap-2 text-red-500 text-sm bg-red-500/10 p-2 rounded-md">
                            <AlertCircle className="h-4 w-4" />
                            <span>{error}</span>
                        </div>
                    )}

                    <div>
                        <label htmlFor="email" className="block text-sm font-medium text-gray-300 mb-2">
                            Email Address
                        </label>
                        <div className="relative">
                            <Mail className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
                            <Input
                                id="email"
                                type="email"
                                value={formData.email}
                                onChange={(e) => onFormDataChange('email', e.target.value)}
                                placeholder="Enter your email"
                                required
                                className="pl-10 bg-black/20 border-gray-700 text-gray-100 placeholder:text-gray-400 focus:bg-black/30 focus:border-gray-600 focus:ring-2 focus:ring-blue-500/50"
                            />
                        </div>
                    </div>

                    <div>
                        <label htmlFor="password" className="block text-sm font-medium text-gray-300 mb-2">
                            Password
                        </label>
                        <div className="relative">
                            <Lock className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
                            <Input
                                id="password"
                                type={showPassword ? "text" : "password"}
                                value={formData.password}
                                onChange={(e) => onFormDataChange('password', e.target.value)}
                                placeholder="Enter your password"
                                required
                                className="pl-10 pr-12 bg-black/20 border-gray-700 text-gray-100 placeholder:text-gray-400 focus:bg-black/30 focus:border-gray-600 focus:ring-2 focus:ring-blue-500/50"
                            />
                            <Button
                                type="button"
                                variant="ghost"
                                size="icon"
                                onClick={onShowPasswordToggle}
                                className="absolute right-2 top-1/2 -translate-y-1/2 h-8 w-8 text-gray-400 hover:text-gray-100 hover:bg-black/20"
                            >
                                {showPassword ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
                            </Button>
                        </div>
                    </div>

                    <div className="flex items-center justify-between">
                        <label className="flex items-center gap-2 cursor-pointer">
                            <input
                                type="checkbox"
                                checked={formData.rememberMe}
                                onChange={(e) => onFormDataChange('rememberMe', e.target.checked)}
                                className="w-4 h-4 rounded border-gray-600 bg-black/30 text-blue-600 focus:ring-blue-500/20 focus:ring-2"
                            />
                            <span className="text-sm text-gray-300">Remember me</span>
                        </label>
                        <Button
                            type="button"
                            variant="ghost"
                            onClick={onForgotPassword}
                            className="text-sm text-gray-400 hover:text-gray-100 hover:bg-black/20 p-0 h-auto"
                        >
                            Forgot password?
                        </Button>
                    </div>

                    <Button
                        type="submit"
                        disabled={isLoading}
                        className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 text-lg disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {isLoading ? (
                            <div className="flex items-center gap-2">
                                <div className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
                                Signing In...
                            </div>
                        ) : (
                            "Sign In"
                        )}
                    </Button>
                </form>

                <div className="relative my-8">
                    <div className="absolute inset-0 flex items-center">
                        <div className="w-full border-t border-gray-800/50" />
                    </div>
                    <div className="relative flex justify-center text-sm">
                        <span className="px-4 bg-black/40 text-gray-400">or</span>
                    </div>
                </div>

                <div className="text-center space-y-4">
                    <p className="text-gray-400">Don't have an account?</p>
                    <Button
                        type="button"
                        variant="outline"
                        onClick={onCreateAccount}
                        className="w-full border-gray-700 text-gray-300 hover:bg-black/20 hover:border-gray-600 hover:text-gray-100"
                    >
                        Create Account
                    </Button>
                </div>
            </div>
        </div>
    )
}