import { AccountInfo } from "../types/auth"

export const fortniteApi = {
    getGameLevel(accountInfo: AccountInfo | null) : number {
        accountInfo?.AccessToken;
        return 1;
    }
}