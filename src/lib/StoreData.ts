import { writable } from "svelte/store";
import type { IClientNotification } from "./DataTypes";

export const ShowPathModal = writable(false)
export const ShowInstallModal = writable(false)
export const VulnusPath = writable("")
export const VersionsAvailable = writable<string[]>([])
export const LatestVersionsAvailable = writable<string>("")
export const AppNotifications = writable<Map<number,IClientNotification>>(new Map())