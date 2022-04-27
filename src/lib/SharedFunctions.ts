 import { event, http, invoke } from "@tauri-apps/api";
import { ShowInstallModal, VulnusPath } from "./StoreData";
import { get } from 'svelte/store';
import { Data } from "./store";
import { documentDir } from "@tauri-apps/api/path";
import { emit } from "@tauri-apps/api/event";
import {shell} from '@tauri-apps/api'
export function getTagDownload(tag:string) {
	return `https://github.com/beat-game-dev/Vulnus/releases/download/${tag}/Vulnus_Beta_Win.zip`
}
export function getTagFromRef(ref:string) {
	return ref.split('/')[2]
}
export function installVersion(tag,desktop=false) {
	event.emit("client://notification",{title:`Please wait`,data:`Installing version ${tag} may take a few seconds or minutes depending on your connection.`})
	return invoke('install_vulnus',{tag,desktop}).then(_=>{
		console.log("OKI")
		event.emit("client://notification",{title:`version ${tag} has been installed`,data:`Have fun hitting those notes`})
		return _
	},e=>e)
}

export function removeVersion(tag) {
	return invoke("remove_vulnus",{tag}).then(_=>{
		console.log("OKR")
		event.emit("client://notification",{title:`version ${tag} has been removed`,data:`Maybe a different time`})
	})
}

export function versionInstalled(tag:string): Promise<boolean> {
	return invoke<boolean>('check_vulnus_tag',{tag})
}
export function launchVulnus(tag:string) {
	documentDir().then(dir=>{
		let installPath = `${dir}vulnus-launcher/${tag}`
		shell.open(`${installPath}/Vulnus.exe`)
	})
}

export function getLatestVulnusTag() {
	return http.fetch<{tag_name:string}>("https://api.github.com/repos/beat-game-dev/Vulnus/releases/latest").then(v=>v.data.tag_name,e=>e)
}
export function getLatestLauncherTag() : Promise<string> {
	return http.fetch<{tag_name:string}>("https://api.github.com/repos/pozm/vulnus-launcher/releases/latest").then(v=>v.data.tag_name,e=>e)
}