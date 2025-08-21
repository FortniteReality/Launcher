export interface ErrorState {
	isOpen: boolean
	title: string
	message: string
}

export interface InstallState {
  isInstalling: boolean
	progress: number
	location: string
}

export interface VerifyState {
	isVerifying: boolean
	progress: number
	result: "success" | "error" | null
}

export interface UninstallState {
	isUninstalling: boolean
	progress: number
	result: "success" | "error" | null
}