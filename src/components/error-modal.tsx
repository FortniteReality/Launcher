import React from "react"
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from "./ui/dialog"
import { Button } from "./ui/button"
import { AlertCircle } from "lucide-react"

interface ErrorModalProps {
	isOpen: boolean
	onClose: () => void
	title: string
	message: string
}

export const ErrorModal: React.FC<ErrorModalProps> = ({ 
	isOpen, 
	onClose, 
	title, 
	message 
}) => (
	<Dialog open={isOpen} onOpenChange={onClose}>
		<DialogContent className="backdrop-blur-xl bg-black/90 border-red-800/50 text-gray-100">
			<DialogHeader>
				<DialogTitle className="text-red-400 flex items-center gap-2">
					<AlertCircle className="h-5 w-5" />
					{title}
				</DialogTitle>
				<DialogDescription className="text-gray-300">
					{message}
				</DialogDescription>
			</DialogHeader>
			<DialogFooter>
				<Button onClick={onClose} className="bg-red-600 hover:bg-red-700">
					OK
				</Button>
			</DialogFooter>
		</DialogContent>
	</Dialog>
)