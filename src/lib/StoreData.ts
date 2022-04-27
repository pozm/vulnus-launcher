import { writable } from "svelte/store";

export const ShowPathModal = writable(false)
export const ShowInstallModal = writable(false)
export const VulnusPath = writable("")
export const VersionsAvailable = writable<string[]>([])
export const LatestVersionsAvailable = writable<string>("")