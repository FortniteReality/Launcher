export interface AccountInfo {
    AccessToken: string;
    RefreshToken: string;
    AccountId: string;
    DisplayName: string;
}

export interface SignInFormData {
    email: string
    password: string
    rememberMe: boolean
}

export interface AuthState {
    isLoading: boolean
    isTransitioning: boolean
    error: string | null
}