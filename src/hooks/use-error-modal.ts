import { useState, useCallback } from "react"
import { ErrorState } from "../types/launcher"

export const useErrorModal = () => {
	const [errorState, setErrorState] = useState<ErrorState>({
		isOpen: false,
		title: "",
		message: ""
	})

	const showError = useCallback((title: string, message: string) => {
		setErrorState({ isOpen: true, title, message })
	}, [])

	const closeError = useCallback(() => {
		setErrorState({ isOpen: false, title: "", message: "" })
	}, [])

	return {
		errorState,
		showError,
		closeError
	}
}